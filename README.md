<p align="center">
  <a href="https://victorgomez.dev">
    <img alt="Portfolio logo" src="./.github/resources/Animated.gif" width="60" height="60" />
  </a>
</p>

### **General info**
Hi 👋! This is the second version of my website's mailer API (previous one was made with, which serves mainly as the backend for my portfolio's contact page (https://victorgomez.dev/contact). 

Also, it is my first Rusty program! Learned a bunch btw about it during this project, mainly about module organization among other things (which I will defenitely improve in other Rusty projects).

Hope you like it 😄!

---
### **Technologies**
Project is created with:
* Rust: v nightly-2022-07-20 - 2021 edition
* Rocket.rs: v0.5.0-rc.2
* Tokio.rs: v1.20.0
* MongoDB Rust driver: v2.2.2
* Serde: v1.0.137
* For other secondary libs/fmwks, please give a look into the [Cargo.toml's](https://github.com/Vicg853/tumex-svc-backcms-gqlfst-1/blob/main/package.json) "[dependencies]" section

---
### **Setup**
  > Note: You must have the Rust ngihtly toolchain installed on your environment (version "nightly-2022-07-20" preferably)
  > Recomended: to have ``cargo-make`` and ``fleet.rs`` installed

  *After downloading/cloning the repository and assuring yourself you are allowed to copy this repo...*

  * **Before running...**
    > ... fill up an ``.env`` file with the following variables
  ```.env
    MS_DB_CLUST_USR= 
    CMS_DB_CLUST_PASS=
    CMS_DB_CLUST_URI=
   
    #Msgs db related
    CMS_MSG_DB_NAME=
   
    #Auth0 tennat config
    TENNANT_ENDPOINT=
    CURR_AUDIENCE=
  ```

  * **...Development**
  ```bash
   # With cargo-make and fleet installed
   cd ./tumex-svc-backcms-gqlfst-1
   cargo make dev
   
   # Without cargo-watch and fleet
   CARGO_TARGET_DIR=./target/dev cargo run --profile dev --out-dir ./target/dev 
   
   # Note that specifying the the cargo target dir is totally optional. 
   # It just helps prevent rust_analyser server, from conflicting with the build target
   # hence causing both to take more time to compile/analyze the codebase
  ```

  * **...Production**
  ```bash
   # With cargo make
   cd ./tumex-svc-backcms-gqlfst-1
   cargo make start 
   # No need to run the build taks as start task takes it as requirement, thefore build is executed automatically before running 
   
   # Without cargo make
   CARGO_TARGET_DIR=./build cargo build --release -Z unstable-options --out-dir ./build
   ## Then
   ./build/rust-mailer-api

   # Again: specifying out dir is totally optional! (if you remove it, make sure you also remove "-Z unstable-options", which is required when specifying builds out dir)
  ```

  * Access:
  **Voilà, you should have access to the server via [``` localhost:8000 ```](http://localhost:8000)**

### **Credits**
Contributors: [@vicg853](https://github.com/Vicg853)

Special thanks to contributors/teams: 
- at Rust org for the amazing lang and experience!
- SergioBenitez for the amazing server project 
- Tokio community for the async runtime
- and all the teams/contributors on other packages used here
