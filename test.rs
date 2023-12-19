fn main() {
    let mut og_str = "/home?k1=v1&k2=v2";

    println!("og_str: {:?}", &og_str);
  
    let mut query = None;

    if let Some((_og_str, _query)) = test_option_return(&og_str){
       query = Some(_query);
       og_str = _og_str;
    }

    println!("og_str: {:?}, query: {:?}", &og_str, query);
}

fn test_option_return(value: &str) -> Option<(&str, &str)> {
   let index = value.find('?');
   if index.is_some() {
       let i = index.unwrap();
       return Some((&value[..i], &value[i+1..]));
    }
   None
}
