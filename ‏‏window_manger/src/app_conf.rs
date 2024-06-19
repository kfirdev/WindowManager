use std::collections::HashMap;
use windows::Win32::Foundation;
use windows::Win32::UI::{Input::KeyboardAndMouse,WindowsAndMessaging};

pub struct App{
	app: Option<Foundation::HWND>,
}

impl App {
	pub fn new(id: i32,key: &String,key_num_map:&HashMap<String,u32>) -> Result<App,String>{
		if let Some((hk,k)) = key.split_once("+"){
			let hk_n = match hk{
				"Alt" => 0x0001,
				"Ctr" => 0x0002,
				"Shf" => 0x0004,
				"Win" => 0x0008,
				_ => return Err(String::from("Not a proper start")),
            	} | 0x4000;


			let kn = match key_num_map.get(k){
				Some(val) => val,
				None => return Err(String::from("does not exist"))
			};

			let ok = unsafe{KeyboardAndMouse::RegisterHotKey(None,id,KeyboardAndMouse::HOT_KEY_MODIFIERS(hk_n),*kn)};
			match ok {
                Ok(_) => (),
				Err(e) => return Err(e.message() + " " +key)
            }
		}


		Ok(App{app: None})
	}
	pub fn handle(&mut self) -> bool{
		match self.app{
			Some(app) if unsafe{WindowsAndMessaging::IsWindow(app).as_bool()} => unsafe{WindowsAndMessaging::SetForegroundWindow(app)}.as_bool(),
			_ => {
				self.app = Some(unsafe{WindowsAndMessaging::GetForegroundWindow()});
				true
			}
		}
	}
	pub fn reset(&mut self){
		self.app = None;
	}
    
}
