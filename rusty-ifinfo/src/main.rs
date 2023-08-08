use clatox_rtnetlink::netlink::*;
use clatox_rtnetlink::rtnetlink::*;

fn show_link() {
    let mut socket = Socket::connect_to_kernel(Protocol::Route)
        .expect("unable to open netlink socket");

    let message = Message::new(
        Flags::new(&[Flag::Request, Flag::Dump]),
        // Since we are requesting Dump, the args below don't really matter
        GetLink(InterfaceInfoMessage::new(
            ArpHardware::Ethernet,
            0,
            InterfaceFlags::new(&[]),
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
                Address(addr) => println!("  interface address: {:?}", addr),
                Broadcast(brd) => println!("  interface broadcast: {:?}", brd),
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
                Stats64(stats) => println!("  interface stats: {:#?}", stats),
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
        Flags::new(&[Flag::Request, Flag::Dump]),
        // Since we are requesting Dump, the args below don't really matter
        GetAddress(InterfaceAddressMessage::new(
            clatox_rtnetlink::rtnetlink::AddressFamily::Inet,
            0,
            AddressFlags::new(&[]),
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
                Address(addr) => println!("  interface address: {:?}", addr),
                Local(addr) => println!("  interface local address: {:?}", addr),
                Label(label) => println!("  interface label: {}", label),
                CacheInfo(cache) => println!("  interface cache info: {:#?}", cache),
                Flags(f) => println!("  interface flags: {}", f),
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

fn main() {
    println!("Printing info dumped with RTM_GETLINK\n=====================================");
    show_link();
    
    println!("");

    println!("Printing info dumped with RTM_GETADDR\n=====================================");
    show_addr();
}
