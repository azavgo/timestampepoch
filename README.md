## Library to convert date to UNIX timestamp and to convert UNIX timestamp to date 

### How to use this library: 
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    uuidv4 = {git = "https://github.com/azavgo/timestampepoch", branch = "main"}
```
2. Generate UNIX timestamp:  
```Rust
    use timestampepoch::*;

    fn main(){
        let date: Date = Date::new(25, 9, 2022);
        println!("UNIX timestamp for the date 25 September 2022 is {}", date.timestamp().unwrap()); 
    }
```
3. Date corresponding UNIX timestamp:  
```Rust
    use timestampepoch::*;

    fn main(){
        let date = Date::date_timestamp(89998300800);
        println!("Date corresponding to the UNIX timestamp 89998300800 is {}", date); 
    }
```