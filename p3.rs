fn main(){
	println!("Hello,world!");
	let s:i32=5;
	call_someone(s);
	println!("{}",s);

}
fn call_someone(s1: i32)
{
	println!("{}",s1);
}