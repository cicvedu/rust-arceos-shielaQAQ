// // pub use driver_net::EthernetAddress;
// // pub use driver_net::NetBufPtr;
// // pub use driver_net::DevError;
//  //pub use driver_common::DevResult;
// // pub use driver_net::NetDriverOps;
// //extern crate driver_net;
// use driver_net::{EthernetAddress, NetBufPtr, DevError, NetDriverOps, DevResult};

// pub struct NetFilter<T> {
//     pub inner: T,
// }

// impl<T> driver_common::BaseDriverOps for NetFilter<T>
// where
//     T: driver_common::BaseDriverOps,
// {
//     // Implement the required methods from BaseDriverOps using the inner device.
//     // For example:
//     fn device_name(&self) -> &str {
//         self.inner.device_name()
//     }

//     fn device_type(&self) -> driver_common::DeviceType {
//         self.inner.device_type()
//     }

//     // Implement other methods from BaseDriverOps using the inner device.
//     // ...
// }

// impl<T: NetDriverOps> NetDriverOps for NetFilter<T> {
//     fn mac_address(&self) -> EthernetAddress {
//         self.inner.mac_address()
//     }

//     fn can_transmit(&self) -> bool {
//         self.inner.can_transmit()
//     }

//     fn can_receive(&self) -> bool {
//         self.inner.can_receive()
//     }

//     fn rx_queue_size(&self) -> usize {
//         self.inner.rx_queue_size()
//     }

//     fn tx_queue_size(&self) -> usize {
//         self.inner.tx_queue_size()
//     }

//     fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
//         //let result = self.inner.recycle_rx_buffer(rx_buf);
//         warn!("Filter: receive len [{:?}]", rx_buf.packet_len());
//         self.inner.recycle_rx_buffer(rx_buf)
//         // if result.is_ok() {
//         //     warn!("Recycled receive buffer");
//         // } else {
//         //     warn!("Failed to recycle receive buffer: {:?}", result);
//         // }
        
//         // result
//     }

//     fn recycle_tx_buffers(&mut self) -> DevResult {
//         let result = self.inner.recycle_tx_buffers();
//         //self.inner.recycle_tx_buffers();
//         // if result.is_ok() {
//         //     warn!("Recycled transmit buffers");
//         // } else {
//         //     warn!("Failed to recycle transmit buffers: {:?}", result);
//         // }
//         //warn!("Filter: transmit len [{}]", &result.unwrap().packet_len());
//         result
//     }

//     fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        
//         //warn!("Filter: transmit len [{}]", packet_len(&tx_buf));
//         //let result2 = tx_buf.clone();
        
//         warn!("Filter: transmit len [{}]", &tx_buf.packet_len());
//         //let result = self.inner.transmit(tx_buf);
//         self.inner.transmit(tx_buf)
//         //result
//     }

//     // fn receive(&mut self) -> DevResult<NetBufPtr> {
        
//     //     let receive_result = self.inner.receive();
//     //     let binding = receive_result.unwrap();
//     //     let rxx_buf = binding.packet().clone();
//     //     warn!("Received packet with length: {}", rxx_buf.len());
//     //     // match &receive_result {
//     //     //     Ok(rxx_buf) => {
//     //     //         warn!("Received packet with length: {}", rxx_buf.packet_len());
//     //     //     },
//     //     //     Err(err) => {
//     //     //         warn!("No incoming packets");

//     //     //     }
//     //     // }
//     //     receive_result
//     // }
//     fn receive(&mut self) -> DevResult<NetBufPtr> {
//         let receive_result = self.inner.receive();
//         //let receive_result2 = self.inner.receive();
//         //warn!("Filter: receive len [{:?}]", packet_len(&receive_result.unwrap()));
//         // match &receive_result {
//         //     Ok(rx_buf) => {
//         //         warn!("Filter: receive len [{:?}]", &rx_buf.packet_len());
//         //     },
//         //     Err(err) => {
//         //         warn!("No incoming packets");
//         //     }
//         // }
    
//         receive_result
//     }

//     fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
//         //let result = self.inner.alloc_tx_buffer(size);
//         self.inner.alloc_tx_buffer(size)


//         //result
//     }
// }


// pub use driver_net::EthernetAddress;
// pub use driver_net::NetBufPtr;
// pub use driver_net::DevError;
// pub use driver_net::NetDriverOps;
//extern crate driver_net;
// use driver_net::{EthernetAddress, NetBufPtr, NetDriverOps, DevResult};
use cfg_if::cfg_if;
pub struct NetFilter<T> {
    pub inner: T,
}
cfg_if! {
    if #[cfg(feature = "net")]{

    
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
    
        impl<T: driver_net::NetDriverOps> driver_net::NetDriverOps for NetFilter<T> {
            fn mac_address(&self) -> driver_net::EthernetAddress {
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
    
            fn recycle_rx_buffer(&mut self, rx_buf: driver_net::NetBufPtr) -> driver_net::DevResult {
                //let result = self.inner.recycle_rx_buffer(rx_buf);
                warn!("Filter: receive len [{:?}]", rx_buf.packet_len());
                self.inner.recycle_rx_buffer(rx_buf)
                // if result.is_ok() {
                //     warn!("Recycled receive buffer");
                // } else {
                //     warn!("Failed to recycle receive buffer: {:?}", result);
                // }
                
                // result
            }
    
            fn recycle_tx_buffers(&mut self) -> driver_net::DevResult {
                let result = self.inner.recycle_tx_buffers();
                //self.inner.recycle_tx_buffers();
                // if result.is_ok() {
                //     warn!("Recycled transmit buffers");
                // } else {
                //     warn!("Failed to recycle transmit buffers: {:?}", result);
                // }
                //warn!("Filter: transmit len [{}]", &result.unwrap().packet_len());
                result
            }
    
            fn transmit(&mut self, tx_buf: driver_net::NetBufPtr) -> driver_net::DevResult {
                
                //warn!("Filter: transmit len [{}]", packet_len(&tx_buf));
                //let result2 = tx_buf.clone();
                
                warn!("Filter: transmit len [{}]", &tx_buf.packet_len());
                //let result = self.inner.transmit(tx_buf);
                self.inner.transmit(tx_buf)
                //result
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
            fn receive(&mut self) -> driver_net::DevResult<driver_net::NetBufPtr> {
                let receive_result = self.inner.receive();
                //let receive_result2 = self.inner.receive();
                //warn!("Filter: receive len [{:?}]", packet_len(&receive_result.unwrap()));
                // match &receive_result {
                //     Ok(rx_buf) => {
                //         warn!("Filter: receive len [{:?}]", &rx_buf.packet_len());
                //     },
                //     Err(err) => {
                //         warn!("No incoming packets");
                //     }
                // }
            
                receive_result
            }
    
            fn alloc_tx_buffer(&mut self, size: usize) -> driver_net::DevResult<driver_net::NetBufPtr> {
                //let result = self.inner.alloc_tx_buffer(size);
                self.inner.alloc_tx_buffer(size)
    
    
                //result
            }
        }
    }
}
