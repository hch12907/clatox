use std::net::IpAddr;

use clatox_rtnetlink::netlink::*;
use clatox_rtnetlink::rtnetlink::*;
use clatox_rtnetlink::rtnetlink::RouteType;

fn remove_line_with_brace(output: String) -> String {
    output
        .split('\n')
        .filter(|x| !x.contains('{') && !x.contains('}'))
        .fold(String::new(), |acc, x| acc + "\n" + x.strip_suffix(',').unwrap())
}

fn pretty_address(output: &[u8]) -> String {
    match output.len() {
        4 => IpAddr::from(<[u8; 4]>::try_from(output).unwrap()).to_string(),
        6 => output.iter().map(|x| format!("{x:x}")).collect::<Vec<_>>().join(":"),
        16 => IpAddr::from(<[u8; 16]>::try_from(output).unwrap()).to_string(),
        _ => format!("{:?}", output),
    }
}

fn show_link() {
    let mut socket = Socket::connect_to_kernel(Protocol::Route)
        .expect("unable to open netlink socket");

    let message = Message::new(
        Flags::Request | Flags::Dump,
        // Since we are requesting Dump, the args below don't really matter
        GetLink(InterfaceInfoMessage::new(
            ArpHardware::Ethernet,
            0,
            InterfaceFlags::empty(),
            vec![]
        ))
    );

    socket.send_message(&message)
        .expect("unable to send netlink message");

    let received = socket.receive_message::<NewLink>()
        .expect("unable to receive a netlink message");

    let print_a_payload = |payload: &InterfaceInfoMessage| {
        for attr in payload.attributes() {
            use InterfaceInfoAttribute::*;
            match attr {
                InterfaceName(name) => println!("  interface name: {}", name),
                Address(addr) => println!("  interface address: {}", pretty_address(addr)),
                Broadcast(brd) => println!("  interface broadcast: {}", pretty_address(brd)),
                MTU(mtu) => println!("  interface MTU: {}", mtu),
                MinMTU(mtu) => println!("  interface minimum MTU: {}", mtu),
                MaxMTU(mtu) => println!("  interface maximum MTU: {}", mtu),
                GroMaxSize(gro) => println!("  interface maximum GRO size: {}", gro),
                GsoMaxSize(gso) => println!("  interface maximum GSO size: {}", gso),
                TsoMaxSize(tso) => println!("  interface maximum TSO size: {}", tso),
                AllMulti(all) => println!("  interface is in all-multi mode: {}", (*all > 0)),
                ParentDeviceName(name) => println!("  interface parent device name: {}", name),
                ParentDeviceBusName(name) => println!("  interface parent device bus name: {}", name),
                InterfaceAlias(name) => println!("  interface alias: {}", name),
                Stats64(stats) =>
                    println!("  interface stats: {}", remove_line_with_brace(format!("{:#?}", stats))),
                _ => (),
            }
        }
        println!();
    };

    if let ReceivedMessage::Multipart(messages) = received {
        println!("netlink returned a multipart message:");

        for message in messages {
            let content = message.payload();
            println!("  interface index: {}", content.0.index());            
            print_a_payload(&content.0);
        }
    } else if let ReceivedMessage::Message(message) = received {
        println!("netlink returned a message:");

        let content = message.payload();
        println!("  interface index: {}", content.0.index());
        print_a_payload(&content.0);
    } else if let ReceivedMessage::Error(err) = received {
        println!("netlink returned an error message:");
        println!("{:?}", err);
    };
}

fn show_addr() {
    let mut socket = Socket::connect_to_kernel(Protocol::Route)
        .expect("unable to open netlink socket");

    let message = Message::new(
        Flags::Request | Flags::Dump,
        // Since we are requesting Dump, the args below don't really matter
        GetAddress(InterfaceAddressMessage::new(
            AddressFamily::Unspecified,
            0,
            AddressFlags::empty(),
            vec![]
        ))
    );

    socket.send_message(&message)
        .expect("unable to send netlink message");

    let received = socket.receive_message::<NewAddress>()
        .expect("unable to receive a netlink message");

    let print_a_payload = |content: &InterfaceAddressMessage| {
        for attr in content.attributes() {
            use InterfaceAddressAttribute::*;
            match attr {
                Address(addr) => println!("  interface address: {}", pretty_address(addr)),
                Local(addr) => println!("  interface local address: {}", pretty_address(addr)),
                Label(label) => println!("  interface label: {}", label),
                CacheInfo(cache) =>
                    println!("  interface cache info: {}", remove_line_with_brace(format!("{:#?}", cache))),
                Flags(f) => println!("  interface flags: {:?}", f),
                _ => (),
            }
        }
        println!();
    };

    if let ReceivedMessage::Multipart(messages) = received {
        println!("netlink returned a multipart message:");

        for message in messages {
            let content = message.payload();
            println!("  interface index: {}", content.0.index());
            print_a_payload(&content.0);
        }
    } else if let ReceivedMessage::Message(message) = received {
        println!("netlink returned a message:");

        let content = message.payload();
        println!("  interface index: {}", content.0.index());
        print_a_payload(&content.0);
    } else if let ReceivedMessage::Error(err) = received {
        println!("netlink returned an error message:");
        println!("{:?}", err);
    }
}

fn show_route() {
    let mut socket = Socket::connect_to_kernel(Protocol::Route)
        .expect("unable to open netlink socket");

    let message = Message::new(
        Flags::Request | Flags::Dump,
        // Since we are requesting Dump, the args below don't really matter
        GetRoute(RouteMessage::new(
            AddressFamily::Unspecified,
            0,
            0,
            0,
            RouteTable::Default,
            RouteProtocol::Unspecified,
            RouteScope::Host,
            RouteType::Unspecified,
            RouteFlags::empty(),
            vec![],
        ))
    );

    socket.send_message(&message)
        .expect("unable to send netlink message");

    let received = socket.receive_message::<NewRoute>()
        .expect("unable to receive a netlink message");

    let print_a_payload = |content: &RouteMessage| {
        println!("  route installer: {:?}", content.protocol());
        println!("  route destination prefix len: {:?}", content.dst_len());
        println!("  route source prefix len: {:?}", content.src_len());

        for attr in content.attributes() {
            use RouteAttribute::*;
            match attr {
                Destination(addr) => println!("  route destination: {}", pretty_address(addr)),
                Source(addr) => println!("  route source: {}", pretty_address(addr)),
                IncomingInterface(iif) => println!("  route incoming interface: {}", iif),
                OutgoingInterface(oif) => println!("  route outgoing interface: {}", oif),
                Gateway(addr) => println!("  route gateway: {}", pretty_address(addr)),
                Metrics(met) =>  println!("  route metrics: {}", met),
                _ => (),
            }
        }
        println!();
    };

    if let ReceivedMessage::Multipart(messages) = received {
        println!("netlink returned a multipart message:");

        for message in messages {
            let content = message.payload();
            print_a_payload(&content.0);
        }
    } else if let ReceivedMessage::Message(message) = received {
        println!("netlink returned a message:");

        let content = message.payload();
        print_a_payload(&content.0);
    } else if let ReceivedMessage::Error(err) = received {
        println!("netlink returned an error message:");
        println!("{:?}", err);
    }
}

fn main() {
    println!("Printing info dumped with RTM_GETLINK\n=====================================");
    show_link();
    
    println!("");

    println!("Printing info dumped with RTM_GETADDR\n=====================================");
    show_addr();

    println!("");
    println!("Printing info dumped with RTM_GETROUTE\n=====================================");
    show_route();
}
