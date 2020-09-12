use crate::mal::env::env_new;
use crate::mal::env::Env;
use crate::mal::rep;
use crate::mal::types::format_error;
use crate::print;
use crate::println;
use crate::task::keyboard::ScancodeStream;
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

pub async fn mal_repl() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    // 输入字符串缓存
    let mut downContrl: bool = false;

    // 初始化环境
    let kernel_env: Env = env_new(None);
    use crate::mal::core::load_core;
    load_core(&kernel_env);
    head();
    print!("IN:");
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        //todo 不同的值不同处理
                        match character {
                            'c' => {
                                if downContrl {
                                    // 如果遇到了 Ctrl+C 清空输入换行
                                    crate::stdio::STDIN.clear();
                                    println!();
                                    downContrl = false;
                                } else {
                                    crate::stdio::STDIN.push(character);
                                    print!("{}", character);
                                }
                            }
                            '\n' => {
                                println!("runing: {}", crate::stdio::STDIN.to_string().as_str());
                                match rep(crate::stdio::STDIN.to_string().as_str(), &kernel_env) {
                                    Ok(out) => println!("OUT:{}", out),
                                    Err(e) => println!("OUT:{}", format_error(e)),
                                }
                                crate::stdio::STDIN.clear();
                                println!();
                                print!("IN:");
                            }
                            _ => {
                                crate::stdio::STDIN.push(character);
                                print!("{}", character);
                            }
                        }
                    }
                    DecodedKey::RawKey(key) => match key {
                        KeyCode::Backspace | KeyCode::Delete => {
                            crate::stdio::STDIN.back_spacse();
                            // todo 要显示的内容也要删除
                        }
                        KeyCode::ControlLeft | KeyCode::ControlRight => {
                            downContrl = true;
                        }
                        KeyCode::Enter => {
                            println!("runing: {}", crate::stdio::STDIN.to_string().as_str());
                            // 程序代码读到 mal 中求值
                            match rep(crate::stdio::STDIN.to_string().as_str(), &kernel_env) {
                                Ok(out) => println!("OUT:{}", out),
                                Err(e) => println!("OUT:{}", format_error(e)),
                            }
                            crate::stdio::STDIN.clear();
                            println!();
                            print!("IN:");
                        }
                        _ => print!("{:?}", key),
                    },
                }
            }
        }
    }
}

pub fn head() {
    println!(
        "
         **   **     **   ****     **   ********   *******  
        /**  /**    /**  /**/**   /**  /**/////   /**////** 
        /**  /**    /**  /**//**  /**  /**        /**   /** 
        /**  /**    /**  /** //** /**  /*******   /*******  
        /**  /**    /**  /**  //**/**  /**////    /**///**  
    **  /**  /**    /**  /**   //****  /**        /**  //** 
   //*****   //*******   /**    //***  /********  /**   //**
    /////     ///////    //      ///   ////////   //     // 
      *******    ********
     **/////**  **////// 
    **     //**/**       
   /**      /**/*********
   /**      /**////////**
   //**     **        /**                                    version: V0.0.1
    //*******   ********                                     since @ 2019
     ///////   ////////                                      made by zhouzihao "
    );
    println!("Weclome to my page:https://github.com/zzhgithub/juner_os");
    println!("since:2019");
    println!("you can use MAL a small lisp!");
    println!();
    let mut i = 0;
    loop {
        if i < 4 {
            println!();
            i = i + 1;
        } else {
            break;
        }
    }
}
