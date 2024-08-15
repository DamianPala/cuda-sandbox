use cudarc::driver::*;


fn ada(dev: &CudaDevice) {
    let mem = dev.alloc_zeros::<f32>(3).unwrap(); // Error method not found in `&CudaDevice`
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev = CudaDevice::new(0)?;

    let mem = dev.alloc_zeros::<f32>(3).unwrap(); // Works
    ada(&dev);


    println!("Successfully.");

    Ok(())
}
