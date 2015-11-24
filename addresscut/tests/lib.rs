extern crate addresscut;

#[test]
fn it_works() {
	let mut v:Vec<String> = Vec::new();
	v.push("中文".to_string());
	let s = match v.get(0) {
		None => panic!("couldn't be happen!"),
		Some(s) => s
	};
	assert!(false, format!("{:?},{}", v, s));
}
