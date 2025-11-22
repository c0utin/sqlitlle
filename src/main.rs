fn main() -> color_eyre::Result<()> {
   color_eyre::install()?;

   let local_exe = glommio::LocalExecutor::default();
   local_exe.run(async{
       println!("Hello, world!");
   });
   println!("Hello, world!");
   Ok(())
}
