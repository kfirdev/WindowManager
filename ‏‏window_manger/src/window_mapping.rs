use crate::app_conf::App;
use std::collections::HashMap;
use windows::Win32::UI::WindowsAndMessaging;

pub struct WindowMapping{
	apps: HashMap<i32,App>,
	id_reset: i32
}

impl WindowMapping{
	pub fn new(mapping: Vec<String>,reset_key: String) -> Result<WindowMapping, String>{
		let key_num_map: HashMap<String, u32> = ('a'..='z').into_iter().enumerate().map(|(i,l)| (l.to_string(),65+i as u32)).collect();

		let mut apps: HashMap<i32,App> = HashMap::new();
		let mut app;
		let mut id_reset = 0;
		for (i,map) in mapping.into_iter().enumerate(){
			app = match App::new(i as i32,&map,&key_num_map){
            	Ok(val) => val,
				Err(e) => return Err(e),
            };
			apps.insert(i as i32,app);
			id_reset = i as i32;
		}
		id_reset += 1;
		_ = match App::new(id_reset as i32,&reset_key,&key_num_map){
        	Ok(val) => val,
			Err(e) => return Err(e),
        };

		Ok(WindowMapping{apps, id_reset})
	}
	pub fn run(&mut self) -> Result<String,String>{
	    let mut msg;
		let mut status;
		let mut final_msg;
		let mut app;

		loop{
			msg = std::mem::MaybeUninit::<WindowsAndMessaging::MSG>::uninit();
		    status = unsafe { WindowsAndMessaging::GetMessageW(msg.as_mut_ptr(), None, WindowsAndMessaging::WM_NULL, WindowsAndMessaging::WM_HOTKEY) };

		    final_msg = unsafe { msg.assume_init() };

			if status.as_bool(){
				if (final_msg.wParam.0 as i32) == self.id_reset{
					self.reset();
					continue;
				}
				app = match self.apps.get_mut(&(final_msg.wParam.0 as i32)){
					Some(val) => val,
					None => return Err(String::from("No such app")),
                    
                };
				match app.handle(){
					false => return Err(String::from("can't set foreground")),
					_ => ()
                    
                }
			}
	
		}

	}
	fn reset(&mut self){
		for (_,v) in self.apps.iter_mut(){
			v.reset();
		}
	}
}
