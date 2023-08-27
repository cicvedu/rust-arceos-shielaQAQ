//use driver_net::NetDriverOps;
use driver_net::EthernetAddress;
use driver_net::NetBufPtr;
use driver_net::DevError;
//use driver_common::{BaseDriverOps, DevResult, DeviceType};
use driver_common::DevResult;
// pub struct NetFilter<T> {
//     pub inner: T,
// }

pub use driver_net::NetDriverOps;

pub struct NetFilter<T> {
    pub inner: T,
}

impl<T> driver_common::BaseDriverOps for NetFilter<T>
where
    T: driver_common::BaseDriverOps,
{
    // Implement the required methods from BaseDriverOps using the inner device.
    // For example:
    fn device_name(&self) -> &str {
        self.inner.device_name()
    }

    fn device_type(&self) -> driver_common::DeviceType {
        self.inner.device_type()
    }

    // Implement other methods from BaseDriverOps using the inner device.
    // ...
}

impl<T: NetDriverOps> NetDriverOps for NetFilter<T> {
    fn mac_address(&self) -> EthernetAddress {
        self.inner.mac_address()
    }

    fn can_transmit(&self) -> bool {
        self.inner.can_transmit()
    }

    fn can_receive(&self) -> bool {
        self.inner.can_receive()
    }

    fn rx_queue_size(&self) -> usize {
        self.inner.rx_queue_size()
    }

    fn tx_queue_size(&self) -> usize {
        self.inner.tx_queue_size()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        //let result = self.inner.recycle_rx_buffer(rx_buf);
        self.inner.recycle_rx_buffer(rx_buf)
        // if result.is_ok() {
        //     warn!("Recycled receive buffer");
        // } else {
        //     warn!("Failed to recycle receive buffer: {:?}", result);
        // }

        // result
    }

    fn recycle_tx_buffers(&mut self) -> DevResult {
        //let result = self.inner.recycle_tx_buffers();
        self.inner.recycle_tx_buffers()
        // if result.is_ok() {
        //     warn!("Recycled transmit buffers");
        // } else {
        //     warn!("Failed to recycle transmit buffers: {:?}", result);
        // }

        // result
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        warn!("Transmit packet with length: {}", tx_buf.packet_len());
        let result = self.inner.transmit(tx_buf);
        //warn!("Transmit packet with length: {}", result.packet_len());
        
        result
    }

    // fn receive(&mut self) -> DevResult<NetBufPtr> {
        
    //     let receive_result = self.inner.receive();
    //     let binding = receive_result.unwrap();
    //     let rxx_buf = binding.packet().clone();
    //     warn!("Received packet with length: {}", rxx_buf.len());
    //     // match &receive_result {
    //     //     Ok(rxx_buf) => {
    //     //         warn!("Received packet with length: {}", rxx_buf.packet_len());
    //     //     },
    //     //     Err(err) => {
    //     //         warn!("No incoming packets");

    //     //     }
    //     // }
    //     receive_result
    // }
    fn receive(&mut self) -> DevResult<NetBufPtr> {
        let receive_result = self.inner.receive();
    
        match &receive_result {
            Ok(rx_buf) => {
                warn!("Received packet with length: {}", rx_buf.packet_len());
            },
            Err(err) => {
                warn!("No incoming packets");
            }
        }
    
        receive_result
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        //let result = self.inner.alloc_tx_buffer(size);
        self.inner.alloc_tx_buffer(size)


        //result
    }
}


        //let rx_buf = receive_result.unwrap();
       // warn!("Received packet with length: {}", self.inner.rx_buffers.packet_len());
       //warn!("Received packet with length: {}", receive_result.as_ref().unwrap().packet_len());
       
        // let rx_buf = &receive_result.unwrap().packet_len();
        // warn!("Received packet with length: {}", rx_buf);
