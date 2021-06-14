pub mod core{
  #[derive(Clone, Debug, Eq, PartialEq)] pub enum TxRx<T>{ Sent(T), Received(T) }
  pub mod http{
    pub mod response{
      #[derive(Clone, Debug, Eq, PartialEq)] pub struct Core{
        pub address: Option<String>,
        pub code: Code,
        pub headers: std::collections::HashMap<String, String>
      }
      #[derive(Clone, Debug, Eq, PartialEq)] pub enum Code{
        Is200Ok,
        Is304NotModified
      }
    }
    pub mod request{
      #[derive(Clone, Debug, Eq, PartialEq)] pub struct Core{
        pub method: Method,
        pub address: Option<String>,
        pub headers: std::collections::HashMap<String, String>
      }
      impl Core{
        pub fn get_address(&self)->Option<String>{
          Some(())
          .and_then(|v|{
            self.headers.get("Host")
          })
          .and_then(|v|{
            self.address.clone()
            .and_then(|v2| Some((v, v2)))
          })
          .and_then(|v|{
            Some(format!("{}{}", v.0, v.1))
          })
        }
      }
      #[derive(Clone, Debug, Eq, PartialEq)] pub enum Method{
        Get
      }
    }
    #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
      Request(request::Core),
      Response(response::Core),
    }
  }
}
mod test{
  mod common{
    pub mod util{
      pub mod events{
        pub mod compare{
          pub mod process{
            pub mod result{ #[derive(Clone, Debug, Eq, PartialEq)] pub struct Core<T1: Clone + std::fmt::Debug + Eq + PartialEq, T0: Clone + IntoIterator<Item=T1>>{
              pub status: Result<(), ()>,
              pub sut_passed: usize,
              pub sut_passed_expected: usize,
              pub sut_total: usize,
              pub sut_map: Vec<(usize, usize)>,
              pub expects: T0,
              pub sut: T0,
            }}
            pub fn core<T1: Clone + std::fmt::Debug + Eq + PartialEq, T0: Clone + IntoIterator<Item=T1>>(expect: T0, sut: T0)->result::Core<T1, T0>{
              let ret = {
                let ret = sut.clone().into_iter().enumerate().fold((0, Vec::<(usize, usize)>::new()), |p, v|{
                  match {
                    expect.clone().into_iter().nth(p.0)  
                    .and_then(|x| Some(v.1==x) )
                    .and_then(|x| match x { true=>Some((v.0)), false=>None } )
                  }{
                    Some(index)=>(p.0+1, {let mut ret = p.1.clone(); ret.push((p.0, index)); ret }),
                    None=>p
                  }
                });
                let ret = (ret.1.len() == expect.clone().into_iter().fold(0, |p, v| p+1), ret);
                ret
              };
              let ret = result::Core{
                status: match ret.0{ true=>Ok(()), false=>Err(()) },
                sut_passed: ret.1.0,
                sut_passed_expected: expect.clone().into_iter().fold(0, |p, v| p+1),
                sut_total: sut.clone().into_iter().fold(0, |p, v| p+1),
                sut_map: ret.1.1,
                expects: expect,
                sut: sut
              };
              ret
            }
          }
          pub fn core<T1: Clone + std::fmt::Debug + Eq + PartialEq, T0: Clone + std::fmt::Debug + IntoIterator<Item=T1>>(panic_handle: fn(), expect: T0, sut: T0)->process::result::Core<T1, T0>{
            #[derive(Clone, Debug, Eq, PartialEq)] struct Result<T1, T0: Clone + std::fmt::Debug + IntoIterator<Item=T1>>{
              pub status: std::result::Result<(), ()>,
              pub sut_passed: usize,
              pub sut_passed_expected: usize,
              pub sut_total: usize,
              pub sut_map: Vec<(usize, usize)>,
              pub expects: T0,
              pub sut: T0,
            }
            let ret = process::core(expect, sut);
            {
              let ret = ret.clone();
              let ret = Result::<(usize, T1), Vec::<_>>{
                status: ret.status,
                sut_passed: ret.sut_passed,
                sut_passed_expected: ret.sut_passed_expected,
                sut_total: ret.sut_total,
                sut_map: ret.sut_map,
                expects: ret.expects.into_iter().enumerate().collect(),
                sut: ret.sut.into_iter().enumerate().collect(),
              };
              if let Err(_) = ret.status { println!("{:#?}", ret) ; panic_handle() }
            };
            ret
          }
        }
      }
    }
    pub mod sut{
      pub mod abstracts{
        pub mod business{
          pub mod core{
            pub mod subs{
              pub mod process{
                pub mod cache{
                  pub mod find{
                    pub trait Core{ fn run(self, input: String)->(Self, Option<crate::core::http::response::Core>) where Self: Sized; }
                  }
                  pub mod store{ pub trait Core{ fn run(self, input: crate::core::http::response::Core)->(Self,) where Self: Sized; }}
                }
                pub mod between_client{
                  pub mod send { pub trait Core{ fn run(self, input: crate::core::http::response::Core)->(Self,) where Self: Sized; }}
                  pub mod receive { pub trait Core{ fn run(self, input: crate::core::http::request::Core)->(Self,) where Self: Sized; }}
                }
                pub mod between_remote{
                  pub mod send { pub trait Core{ fn run(self, input: crate::core::http::request::Core)->(Self,) where Self: Sized; }}
                  pub mod receive { 
                    pub mod start{ pub trait Core{ fn run(self, input: crate::core::http::response::Core)->(Self,) where Self: Sized; }}
                    pub mod process{ pub mod transform{ pub mod of_304{
                      pub trait Core{
                        fn run(self, input: crate::core::http::response::Core)->(Self, crate::core::http::response::Core) where Self: Sized;
                      }
                    }}}
                  }
                }
                pub trait Core : Sized +
                    cache::find::Core + cache::store::Core
                  + between_client::send::Core + between_client::receive::Core
                  + between_remote::send::Core + between_remote::receive::start::Core + between_remote::receive::process::transform::of_304::Core
                {}
              }
              pub mod settings{
                pub mod transmission{ pub mod strictly_no_renew{ 
                  pub mod get { pub trait Core{ fn run(self)->(Self, bool) where Self: Sized; } }
                  pub mod set { pub trait Core{ fn run(self, input:bool)->(Self,) where Self: Sized; } }
                }}
                pub trait Core : Sized +
                  transmission::strictly_no_renew::get::Core + transmission::strictly_no_renew::set::Core
                {}
              }
            }
            pub trait Core : Sized +
                subs::process::Core
              + subs::settings::Core
            {}
          }
          pub mod test{
            pub mod subs{
              pub mod process{
                pub mod client{ pub mod between_proxy{
                  pub mod send{ pub trait Core { fn run(self, input: crate::core::http::request::Core)->(Self,) where Self: Sized; }}
                  pub mod receive{ pub trait Core { fn run(self, input: crate::core::http::response::Core)->(Self,) where Self: Sized; }}
                }}
                pub mod remote{ pub mod between_proxy{
                  pub mod send{ pub trait Core { fn run(self, input: crate::core::http::response::Core)->(Self,) where Self: Sized; }}
                  pub mod receive{ pub trait Core { fn run(self, input: crate::core::http::request::Core)->(Self,) where Self: Sized; }}
                  pub mod cans{ 
                    pub mod get{ pub trait Core{ fn run(self)->(Self, crate::core::http::response::Core) where Self: Sized; } }
                    pub mod set{ pub trait Core{ fn run(self, input: crate::core::http::response::Core)->(Self,) where Self: Sized; } }
                  }
                }}
                pub trait Core : Sized +
                    client::between_proxy::send::Core + client::between_proxy::receive::Core
                  + remote::between_proxy::send::Core + remote::between_proxy::receive::Core
                  + remote::between_proxy::cans::get::Core + remote::between_proxy::cans::set::Core
                {}
              }
              pub trait Core : Sized + process::Core{}
            }
            pub trait Core : Sized + subs::Core + super::core::Core{}
          }
        }
      }
      pub mod implmnts{
        pub mod flow{
          pub mod core{
            pub mod fns{
              pub mod between_client{ pub mod receive{
                pub fn run<T: crate::test::common::sut::abstracts::business::core::Core>(x: T, input: crate::core::http::request::Core)->(T,){
                  let ret = {
                    let ret = (input.get_address().and_then(|x| Some((x,))), x);
                    let ret = match &ret.0{
                      Some(arg)=>ret,
                      None=>(None, crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core::run(ret.1, input.clone()).0)
                    };
                    ret
                  };
                  let ret = match ret.0{
                    Some(arg)=>{
                      let ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core::run(ret.1, arg.0.clone());
                      let ret = match ret.1{
                        Some(val)=>(Some((arg.0, val)), ret.0),
                        None=>(None, crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core::run(ret.0, input.clone()).0)
                      };
                      ret
                    }, 
                    None=>(None, ret.1)
                  };
                  let ret = match ret.0{
                    Some(arg)=>{
                      let ret = crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::get::Core::run(ret.1); 
                      let ret = match ret.1{
                        false=>(Some(arg), ret.0),
                        true=>(None, crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core::run(ret.0, arg.1).0)
                      };
                      ret
                    },
                    None=>(None, ret.1)
                  };
                  let ret = match ret.0{
                    Some(arg)=>{
                      let ret = match arg.1.headers.get("Etag"){
                        Some(val)=>(
                          None,
                          crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core::run(
                            ret.1,
                            crate::core::http::request::Core{
                              headers: {
                                let mut ret = input.clone().headers;
                                ret.insert(String::from("If-Not-Modified"), val.clone());
                                ret
                              },
                              ..input.clone()
                            }
                          ).0
                        ),
                        None=>(Some(arg), ret.1)
                      };
                      ret
                    },
                    None=>(None, ret.1)
                  };
                  let ret = match ret.0{
                    Some(arg)=>{
                      let ret = match arg.1.headers.get("Last-Modified"){
                        Some(val)=>(
                          Option::<()>::None,
                          crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core::run(
                            ret.1,
                            crate::core::http::request::Core{
                              headers: {
                                let mut ret = input.clone().headers;
                                ret.insert(String::from("If-Not-Modified-Since"), val.clone());
                                ret
                              },
                              ..input
                            }
                          ).0
                        ),
                        None=>(None, crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core::run(ret.1, input.clone()).0)
                      };
                      ret
                    },
                    None=>(None, ret.1)
                  };
                  let ret = (ret.1,);
                  ret
                }
              }}
              pub mod between_remote{ 
                pub mod receive{ pub fn run<T: crate::test::common::sut::abstracts::business::core::Core>(x: T, input: crate::core::http::response::Core)->(T,){
                  let mut ret = (input.code.clone(), x);
                  let mut ret = match ret.0{
                    crate::core::http::response::Code::Is200Ok=>{
                      let mut ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::store::Core::run(ret.1, input.clone()).0;
                      let mut ret = crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core::run(ret, input);
                      ret
                    },
                    crate::core::http::response::Code::Is304NotModified=>{
                      let mut ret = crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::process::transform::of_304::Core::run(ret.1, input);
                      let mut ret = crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core::run(ret.0, ret.1);
                      ret
                    }
                  };
                  ret
                }}
                pub mod do_304_transformation{ 
                  pub fn run<T: crate::test::common::sut::abstracts::business::core::Core>(x: T, input: crate::core::http::response::Core)->(T, crate::core::http::response::Core){
                    let ret = (input.address.clone(), x);
                    let ret = match ret.0{
                      Some(arg)=>{
                        let ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core::run(ret.1, arg);
                        let ret = match ret.1{
                          Some(cache)=>{
                            crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::process::transform::of_304::Core::run(ret.0, cache)
                          },
                          None=>(ret.0, input)
                        };
                        ret
                      },
                      None=>(ret.1, input)
                    };
                    ret
                  }
                }
              }
            }
            pub struct Core<T: crate::test::common::sut::abstracts::business::core::Core>{
              pub item: T
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core for Core<T>{
              fn run(mut self, input: String)->(Self, Option<crate::core::http::response::Core>){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::cache::store::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::store::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_client::receive::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = fns::between_client::receive::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::start::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = fns::between_remote::receive::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::process::transform::of_304::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self, crate::core::http::response::Core){
                let ret = fns::between_remote::do_304_transformation::run(self.item, input);
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::get::Core for Core<T>{
              fn run(mut self)->(Self, bool){
                let ret = crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::get::Core::run(self.item);
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::set::Core for Core<T>{
              fn run(mut self, input:bool)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::set::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::settings::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::Core for Core<T>{}
          }
          pub mod test{ pub mod unit{
            pub struct Core<T: crate::test::common::sut::abstracts::business::test::Core>{
              pub item: T
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core for Core<T>{
              fn run(mut self, input: String)->(Self, Option<crate::core::http::response::Core>){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::cache::store::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::store::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core::run(self.item, input.clone());
                self.item = ret.0;
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::receive::Core::run(self, input);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_client::receive::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_client::receive::Core::run(self.item, input.clone());
                self.item = ret.0;
                let ret = crate::test::common::sut::implmnts::flow::core::fns::between_client::receive::run(self, input);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core::run(self.item, input.clone());
                self.item = ret.0;
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::receive::Core::run(self, input);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::start::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::start::Core::run(self.item, input.clone());
                self.item = ret.0;
                let ret = crate::test::common::sut::implmnts::flow::core::fns::between_remote::receive::run(self, input);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::process::transform::of_304::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self, crate::core::http::response::Core){
                let ret = crate::test::common::sut::implmnts::flow::core::fns::between_remote::do_304_transformation::run(self.item, input);
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::get::Core for Core<T>{
              fn run(mut self)->(Self, bool){
                let ret = crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::get::Core::run(self.item);
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::set::Core for Core<T>{
              fn run(mut self, input:bool)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::set::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::settings::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(self.item, input.clone());
                self.item = ret.0;
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_client::receive::Core::run(self, input);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core>  crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::receive::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::receive::Core::run(self.item, input.clone());
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::get::Core for Core<T>{
              fn run(mut self)->(Self, crate::core::http::response::Core){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::get::Core::run(self.item);
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core>  crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::send::Core::run(self.item, input.clone());
                self.item = ret.0;
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::start::Core::run(self, input);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core>  crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::receive::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = self.item;
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::receive::Core::run(ret, input.clone());
                self.item = ret.0;
                let ret = self;
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::get::Core::run(ret);
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::send::Core::run(
                  ret.0,
                  ret.1
                );
                ret
              }
            }
          }}
        }
        pub mod log{
          pub mod test{ pub mod unit{
            pub mod choices{
              pub mod core{
                pub mod process{
                  pub mod cache{
                    pub mod find{
                      #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                        Start(String),
                        Returns(Option<crate::core::http::response::Core>)
                      }
                    }
                    pub mod resend_candidate{
                      pub mod match_criteria{
                        #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                          Etag
                        }
                      }
                      #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                        MatchCriteria(match_criteria::Core),
                        Status(bool)
                      }
                    }
                    #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                      Find(find::Core),
                      ResendCandidate(resend_candidate::Core),
                      Store(crate::core::http::response::Core),
                      
                    }
                  }
                  pub mod between_client{
                    #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                      Send(crate::core::http::response::Core),
                      Receive(crate::core::http::request::Core)
                    }
                  }
                  pub mod between_remote{
                    pub mod receive{
                      pub mod process{
                        pub mod is_304_transformation{
                          #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                            Returned(crate::core::http::response::Core)
                          }  
                        }
                        #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                          Is304Transformation(is_304_transformation::Core)
                        }
                      }
                      #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                        Start(crate::core::http::response::Core),
                        Process(process::Core)
                      }
                    }
                    #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                      Send(crate::core::http::request::Core),
                      Receive(receive::Core)
                    }
                  }
                  #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                    Cache(cache::Core),
                    BetweenClient(between_client::Core),
                    BetweenRemote(between_remote::Core),
                  }
                }
                pub mod settings{
                  pub mod transmission{
                    pub mod strictly_no_renew{
                      #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                        Get(bool),
                        Set(bool)
                      }
                    }
                    #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                      StrictlyNoRenew(strictly_no_renew::Core)
                    }
                  }
                  #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                    Transmission(transmission::Core)
                  }
                }
                #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                  Process(process::Core),
                  Settings(settings::Core),
                }
              }
              pub mod test{
                pub mod client{
                  pub mod between_proxy{
                    #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                      Send(crate::core::http::request::Core),
                      Receive(crate::core::http::response::Core)
                    }
                  }
                  #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                    BetweenProxy(between_proxy::Core)
                  }
                }
                pub mod remote{
                  pub mod between_proxy{
                    #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                      Send(crate::core::http::response::Core),
                      Receive(crate::core::http::request::Core)
                    }
                  }
                  #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                    BetweenProxy(between_proxy::Core)
                  }
                }
                #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                  Client(client::Core),
                  Remote(remote::Core)
                }
              }
              #[derive(Clone, Debug, Eq, PartialEq)] pub enum Core{
                Core(core::Core),
                Test(test::Core)
              }
            }
            pub struct Core<T: crate::test::common::sut::abstracts::business::core::Core>{
              pub log: Vec<choices::Core>,
              pub item: T
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core for Core<T>{
              fn run(mut self, input: String)->(Self, Option<crate::core::http::response::Core>){
                self.log.push(choices::Core::Core(choices::core::Core::Process(choices::core::process::Core::Cache(choices::core::process::cache::Core::Find(choices::core::process::cache::find::Core::Start(input.clone()))))));
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core::run(self.item, input);
                self.log.push(choices::Core::Core(choices::core::Core::Process(choices::core::process::Core::Cache(choices::core::process::cache::Core::Find(choices::core::process::cache::find::Core::Returns(ret.1.clone()))))));
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::cache::store::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::cache::store::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Core(choices::core::Core::Process(choices::core::process::Core::Cache(choices::core::process::cache::Core::Store(input)))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Core(choices::core::Core::Process(choices::core::process::Core::BetweenClient(choices::core::process::between_client::Core::Send(input)))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_client::receive::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_client::receive::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Core(choices::core::Core::Process(choices::core::process::Core::BetweenClient(choices::core::process::between_client::Core::Receive(input)))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Core(choices::core::Core::Process(choices::core::process::Core::BetweenRemote(choices::core::process::between_remote::Core::Send(input)))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::start::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::start::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Core(choices::core::Core::Process(choices::core::process::Core::BetweenRemote(choices::core::process::between_remote::Core::Receive(choices::core::process::between_remote::receive::Core::Start(input))))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::process::transform::of_304::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self, crate::core::http::response::Core){
                let ret = crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::process::transform::of_304::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Core(choices::core::Core::Process(choices::core::process::Core::BetweenRemote(choices::core::process::between_remote::Core::Receive(choices::core::process::between_remote::receive::Core::Process(choices::core::process::between_remote::receive::process::Core::Is304Transformation(choices::core::process::between_remote::receive::process::is_304_transformation::Core::Returned(ret.1.clone()))))))));
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::process::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::get::Core for Core<T>{
              fn run(mut self)->(Self, bool){
                let ret = crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::get::Core::run(self.item);
                self.item = ret.0;
                self.log.push(choices::Core::Core(choices::core::Core::Settings(choices::core::settings::Core::Transmission(choices::core::settings::transmission::Core::StrictlyNoRenew(choices::core::settings::transmission::strictly_no_renew::Core::Get(ret.1))))));
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::set::Core for Core<T>{
              fn run(mut self, input:bool)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::set::Core::run(self.item, input);
                self.item = ret.0;
                self.log.push(choices::Core::Core(choices::core::Core::Settings(choices::core::settings::Core::Transmission(choices::core::settings::transmission::Core::StrictlyNoRenew(choices::core::settings::transmission::strictly_no_renew::Core::Set(input))))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::subs::settings::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::core::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::Core for Core<T>{}
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Test(choices::test::Core::Client(choices::test::client::Core::BetweenProxy(choices::test::client::between_proxy::Core::Send(input)))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::receive::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::receive::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Test(choices::test::Core::Client(choices::test::client::Core::BetweenProxy(choices::test::client::between_proxy::Core::Receive(input)))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::send::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::send::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Test(choices::test::Core::Remote(choices::test::remote::Core::BetweenProxy(choices::test::remote::between_proxy::Core::Send(input)))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::receive::Core for Core<T>{
              fn run(mut self, input: crate::core::http::request::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::receive::Core::run(self.item, input.clone());
                self.item = ret.0;
                self.log.push(choices::Core::Test(choices::test::Core::Remote(choices::test::remote::Core::BetweenProxy(choices::test::remote::between_proxy::Core::Receive(input)))));
                let ret = (self,);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::get::Core for Core<T>{
              fn run(mut self)->(Self, crate::core::http::response::Core){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::get::Core::run(self.item);
                self.item = ret.0;
                let ret = (self, ret.1);
                ret
              }
            }
            impl<T: crate::test::common::sut::abstracts::business::core::Core + crate::test::common::sut::abstracts::business::test::Core> crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core for Core<T>{
              fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(self.item, input);
                self.item = ret.0;
                let ret = (self,);
                ret
              }
            }
          }}
        }
        pub mod core{
          pub mod test{ pub mod unit{
            pub mod internal{
              pub struct Core{
                pub cache: std::collections::HashMap<String, crate::core::http::response::Core>,
                pub can: crate::core::http::response::Core,
                pub settings_transmission_stictly_no_renew: bool
              }
              impl crate::test::common::sut::abstracts::business::core::subs::process::cache::find::Core for Core{
                fn run(mut self, input: String)->(Self, Option<crate::core::http::response::Core>){
                  let ret = self.cache.get(&input).cloned();
                  let ret = (self, ret);
                  ret
                }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::process::cache::store::Core for Core{
                fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                  if let Some(address) = input.address.clone() { self.cache.insert(address, input); }
                  let ret = (self,);
                  ret
                }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::process::between_client::send::Core for Core{
                fn run(mut self, input: crate::core::http::response::Core)->(Self,){ let ret = (self,) ; ret }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::process::between_client::receive::Core for Core{
                fn run(mut self, input: crate::core::http::request::Core)->(Self,){ let ret = (self,) ; ret  }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::process::between_remote::send::Core for Core{
                fn run(mut self, input: crate::core::http::request::Core)->(Self,){ let ret = (self,) ; ret  }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::start::Core for Core{
                fn run(mut self, input: crate::core::http::response::Core)->(Self,){ let ret = (self,) ; ret  }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::process::between_remote::receive::process::transform::of_304::Core for Core{
                fn run(mut self, input: crate::core::http::response::Core)->(Self, crate::core::http::response::Core){
                  let ret = (self, input) ; ret 
                }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::process::Core for Core{}
              impl crate::test::common::sut::abstracts::business::core::Core for Core{}
              impl crate::test::common::sut::abstracts::business::test::Core for Core{}
              impl crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::get::Core for Core{
                fn run(mut self)->(Self, bool){
                  let ret = self.settings_transmission_stictly_no_renew;
                  let ret = (self, ret);
                  ret
                }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::set::Core for Core{
                fn run(mut self, input:bool)->(Self,){
                  self.settings_transmission_stictly_no_renew = input;
                  let ret = (self,);
                  ret
                }
              }
              impl crate::test::common::sut::abstracts::business::core::subs::settings::Core for Core{}
              impl crate::test::common::sut::abstracts::business::test::subs::Core for Core{}
              impl crate::test::common::sut::abstracts::business::test::subs::process::Core for Core{}
              impl crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core for Core{
                fn run(mut self, input: crate::core::http::request::Core)->(Self,){ (self,) }
              }
              impl crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::receive::Core for Core{
                fn run(mut self, input: crate::core::http::response::Core)->(Self,){ (self,) }
              }
              impl crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::send::Core for Core{
                fn run(mut self, input: crate::core::http::response::Core)->(Self,){ (self,) }
              }
              impl crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::receive::Core for Core{
                fn run(mut self, input: crate::core::http::request::Core)->(Self,){ (self,) }
              }
              impl crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::get::Core for Core{
                fn run(mut self)->(Self, crate::core::http::response::Core){
                  let ret = self.can.clone();
                  let ret = (self, ret);
                  ret
                }
              }
              impl crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core for Core{
                fn run(mut self, input: crate::core::http::response::Core)->(Self,){
                  self.can = input;
                  let ret = (self,);
                  ret
                }
              }
            }
            pub mod core{
              pub type Type = crate::test::common::sut::implmnts::flow::test::unit::Core<
                crate::test::common::sut::implmnts::log::test::unit::Core<
                  super::internal::Core
                >
              >;
              pub fn new()->Type{
                let ret = super::internal::Core{
                  cache: Vec::<(String, crate::core::http::response::Core)>::new().iter().cloned().collect(), 
                  can: crate::test::testers::common::http::cern::response(),
                  settings_transmission_stictly_no_renew: false
                };
                let ret = crate::test::common::sut::implmnts::log::test::unit::Core{log: vec![], item: ret};
                let ret = crate::test::common::sut::implmnts::flow::test::unit::Core{
                  item: ret
                };
                ret
              }
            }
          }}
        }
      }
    }
  }
  mod testers{
    pub mod common{
      pub mod http{ pub mod cern{
        pub fn request()->crate::core::http::request::Core{
          crate::core::http::request::Core{
            method: crate::core::http::request::Method::Get,
            address: Some(String::from("/")),
            headers: [
              ("Host", "info.cern.com")
            ].iter().map(|v| (String::from(v.0), String::from(v.1))).collect()
          }
        }
        pub fn response()->crate::core::http::response::Core{
          crate::core::http::response::Core{
            address: Some(String::from("info.cern.com/")),
            code: crate::core::http::response::Code::Is200Ok,
            headers: [
              ("Date",           "Mon, 08 Feb 2021 13:01:29 GMT"),
              ("Server",         "Apache"),
              ("Accept-Ranges",  "bytes"),
              ("Content-Length", "646"),
              ("Connection",     "close"),
              ("Content-Type",   "text/html"),
            ].iter().map(|v| (String::from(v.0), String::from(v.1))).collect()
          }
        }
      }}
    }
    pub mod specifics{
      pub mod proxy{ 
        pub mod plain{
          pub mod strictly_no_renew{
            mod abstracts{
              pub mod core{
                mod steps{
                  pub mod assert{
                    mod subs{
                      pub mod oneset{
                        pub mod choices{ pub enum Core{
                            Client_SendTo_Proxy_Request,
                            Proxy_ReceiveFrom_Client_Request,
                            Proxy_Cache_Returns_None,
                            Proxy_SendTo_Remote_Request,
                            Remote_ReceiveFrom_Proxy_Request,
                            Remote_SendTo_Proxy_Response,
                            Proxy_ReceiveFrom_Remote_Response,
                            Proxy_Cache_Store,
                            Proxy_SendTo_Client_Response,
                            Client_ReceiveFrom_Proxy_Response
                        }}
                        pub const core : &[choices::Core] = {
                          use choices::Core::*;
                          &[
                            Client_SendTo_Proxy_Request,
                            Proxy_ReceiveFrom_Client_Request,
                            Proxy_Cache_Returns_None,
                            Proxy_SendTo_Remote_Request,
                            Remote_ReceiveFrom_Proxy_Request,
                            Remote_SendTo_Proxy_Response,
                            Proxy_ReceiveFrom_Remote_Response,
                            Proxy_Cache_Store,
                            Proxy_SendTo_Client_Response,
                            Client_ReceiveFrom_Proxy_Response
                          ]
                        };
                      }
                      pub mod core{
                        pub mod choices{ pub enum Core{
                          Client_SendTo_Proxy_Request,
                          Proxy_ReceiveFrom_Client_Request,
                          Proxy_Cache_Returns_Some,
                          Proxy_Settings_Transmission_StrictlyNoRenew_Is_True,
                          Proxy_SendTo_Client_Response,
                          Client_ReceiveFrom_Proxy_Response
                        }}
                        pub const core : &[choices::Core] = {
                          use choices::Core::*;
                          &[
                            Client_SendTo_Proxy_Request,
                            Proxy_ReceiveFrom_Client_Request,
                            Proxy_Cache_Returns_Some,
                            Proxy_Settings_Transmission_StrictlyNoRenew_Is_True,
                            Proxy_SendTo_Client_Response,
                            Client_ReceiveFrom_Proxy_Response
                          ]
                        };
                      }
                    }
                    pub mod events{
                      mod subs{
                        pub mod oneset{
                          pub fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::super::super::subs::oneset::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                            p.append(&mut match v{
                              super::super::super::subs::oneset::choices::Core::Client_SendTo_Proxy_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Send;
                                use crate::test::testers::common::http::cern::request;
                                vec![Test(Client(BetweenProxy(Send(request()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_ReceiveFrom_Client_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Receive;
                                use crate::test::testers::common::http::cern::request;
                                vec![Core(Process(BetweenClient(Receive(request()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_Cache_Returns_None=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Find;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::find::Core::Returns;
                                vec![Core(Process(Cache(Find(Returns(None)))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_SendTo_Remote_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Send;
                                use crate::test::testers::common::http::cern::request;
                                
                                vec![Core(Process(BetweenRemote(Send(request()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Remote_ReceiveFrom_Proxy_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Receive;
                                use crate::test::testers::common::http::cern::request;
                                vec![Test(Remote(BetweenProxy(Receive(request()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Remote_SendTo_Proxy_Response=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Send;
                                use crate::test::testers::common::http::cern::response;
                                vec![Test(Remote(BetweenProxy(Send(response()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_ReceiveFrom_Remote_Response=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Start;
                                use crate::test::testers::common::http::cern::response;
                                vec![Core(Process(BetweenRemote(Receive(Start(response())))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_Cache_Store=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Store;
                                use crate::test::testers::common::http::cern::response;
                                vec![Core(Process(Cache(Store(response()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_SendTo_Client_Response=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Send;
                                use crate::test::testers::common::http::cern::response;
                                vec![Core(Process(BetweenClient(Send(response()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Client_ReceiveFrom_Proxy_Response=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Receive;
                                use crate::test::testers::common::http::cern::response;
                                vec![Test(Client(BetweenProxy(Receive(response()))))]
                              },
                            });
                            p
                          }
                        }
                        pub mod core{
                          pub fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::super::super::subs::core::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                            p.append(&mut match v{
                              super::super::super::subs::core::choices::Core::Client_SendTo_Proxy_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Send;
                                use crate::test::testers::common::http::cern::request;
                                vec![Test(Client(BetweenProxy(Send(request()))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_ReceiveFrom_Client_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Receive;
                                use crate::test::testers::common::http::cern::request;
                                vec![Core(Process(BetweenClient(Receive(request()))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_Cache_Returns_Some=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Find;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::find::Core::Returns;
                                use crate::test::testers::common::http::cern::response;
                                vec![Core(Process(Cache(Find(Returns(Some(response()))))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_Settings_Transmission_StrictlyNoRenew_Is_True=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Settings;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::settings::Core::Transmission;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::settings::transmission::Core::StrictlyNoRenew;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::settings::transmission::strictly_no_renew::Core::Get;
                                vec![Core(Settings(Transmission(StrictlyNoRenew(Get(true)))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_SendTo_Client_Response=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Send;
                                use crate::test::testers::common::http::cern::response;
                                vec![Core(Process(BetweenClient(Send(response()))))]
                              },
                              super::super::super::subs::core::choices::Core::Client_ReceiveFrom_Proxy_Response=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Receive;
                                use crate::test::testers::common::http::cern::response;
                                vec![Test(Client(BetweenProxy(Receive(response()))))]
                              },
                            });
                            p
                          }
                        }
                      }
                      fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                        p.append(&mut match v{
                          super::choices::Core::OneSet=>super::subs::oneset::core.iter().map(|v| v.clone()).fold(vec![], subs::oneset::fold),
                          super::choices::Core::Core=>super::subs::core::core.iter().map(|v| v.clone()).fold(vec![], subs::core::fold),
                        });
                        p
                      }
                      pub fn core()->Vec<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                        super::core.iter().fold(vec![], fold)
                      }
                      pub fn function(panic_handler: fn(), sut: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>){
                        crate::test::common::util::events::compare::core(panic_handler, core(), sut);
                      }
                    }
                    pub mod choices{ pub enum Core{
                      OneSet,
                      Core
                    }}
                    const core : &[choices::Core] = {
                      use choices::Core::*;
                      &[
                        OneSet,
                        Core
                      ]
                    };
                  }
                  pub mod choices{ pub enum Core{
                    Arrange_Settings_Transmission_StictlyNoRenew,
                    Act_OneSet,
                    Act_Core,
                    Assert(fn())
                  }}
                  pub const core : &[choices::Core] = &[
                    choices::Core::Arrange_Settings_Transmission_StictlyNoRenew,
                    choices::Core::Act_OneSet,
                    choices::Core::Act_Core,
                    choices::Core::Assert(|| panic!()),
                  ];
                }
                pub struct Runner<TState: crate::test::common::sut::abstracts::business::test::Core>{
                  pub state_new: fn()->TState,
                  pub state_get_log: fn(&TState)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>
                }
                impl<TState: crate::test::common::sut::abstracts::business::test::Core> Runner<TState>{
                  pub fn run(self){
                    steps::core.iter().fold(
                      (self.state_new)(),
                      |p, v|{
                        match v{
                          steps::choices::Core::Arrange_Settings_Transmission_StictlyNoRenew=>{
                            let ret = (p,);
                            let ret = crate::test::common::sut::abstracts::business::core::subs::settings::transmission::strictly_no_renew::set::Core::run(
                              ret.0,
                              true
                            );
                            let ret = ret.0;
                            ret
                          },
                          steps::choices::Core::Act_OneSet=>{
                            let ret = (p,);
                            let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(
                              ret.0,
                              crate::test::testers::common::http::cern::response()
                            );
                            let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(
                              ret.0,
                              crate::test::testers::common::http::cern::request()
                            );
                            let ret = ret.0;
                            ret
                          },
                          steps::choices::Core::Act_Core=>{
                            let ret = (p,);
                            let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(
                              ret.0,
                              crate::test::testers::common::http::cern::response()
                            );
                            let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(
                              ret.0,
                              crate::test::testers::common::http::cern::request()
                            );
                            let ret = ret.0;
                            ret
                          },
                          steps::choices::Core::Assert(panic_handler)=>{ steps::assert::events::function(*panic_handler, (self.state_get_log)(&p)) ; p }
                        }
                      }
                    );
                  }
                }
              }
            }
            pub mod implmnts{
              pub mod test{ pub mod unit{
                pub fn run(){
                  super::super::super::abstracts::core::Runner{
                    state_new: || crate::test::common::sut::implmnts::core::test::unit::core::new(), 
                    state_get_log: |p|{
                      let ret = p;
                      let ret = &ret.item;
                      let ret = ret.log.clone();
                      ret
                    }
                  }.run()
                }
              }}
            }
          }
        }
        pub mod etag{
          pub mod common{
            pub fn response_etag0()->crate::core::http::response::Core{
              let ret = crate::test::testers::common::http::cern::response();
              let ret = crate::core::http::response::Core{
                headers: {
                  let mut ret = ret.headers.clone();
                  ret.insert(String::from("Etag"), String::from("0"));
                  ret
                },
                ..ret
              };
              ret
            }
            pub fn request_etag0()->crate::core::http::request::Core{
              let ret = crate::test::testers::common::http::cern::request();
              let ret = crate::core::http::request::Core{
                headers: {
                  let mut ret = ret.headers.clone();
                  ret.insert(String::from("If-Not-Modified"), String::from("0"));
                  ret
                },
                ..ret
              };
              ret
            }
            pub fn response_304()->crate::core::http::response::Core{
              let ret = crate::test::testers::common::http::cern::response();
              let ret = crate::core::http::response::Core{
                code: crate::core::http::response::Code::Is304NotModified,
                ..ret
              };
              ret
            }
            pub fn response_etag1()->crate::core::http::response::Core{
              let ret = crate::test::testers::common::http::cern::response();
              let ret = crate::core::http::response::Core{
                headers: {
                  let mut ret = ret.headers.clone();
                  ret.insert(String::from("Etag"), String::from("1"));
                  ret
                },
                ..ret
              };
              ret
            }
            pub fn response_lmod0()->crate::core::http::response::Core{
              let ret = crate::test::testers::common::http::cern::response();
              let ret = crate::core::http::response::Core{
                headers: {
                  let mut ret = ret.headers.clone();
                  ret.insert(String::from("Last-Modified"), String::from("Fri, 01 Jan 2021 00:00:01 GMT"));
                  ret
                },
                ..ret
              };
              ret
            }
          }
          pub mod specifics{
            pub mod not_modified{
              mod abstracts{
                pub mod core{
                  mod steps{
                    pub mod assert{
                      mod subs{
                        pub mod oneset{
                          pub mod choices{ pub enum Core{
                            Client_SendTo_Proxy_Request,
                            Proxy_ReceivedFrom_Client_Request,
                            Proxy_SendTo_Remote_Request,
                            Remote_ReceivedFrom_Proxy_Request,
                            Remote_SendTo_Proxy_Response_ETag0,
                            Proxy_ReceivedFrom_Remote_Response_ETag0,
                            Proxy_Cache_Store,
                            Proxy_SendTo_Client_Response_ETag0,
                            Client_ReceivedFrom_Response_ETag0,
                          }}
                          pub const core : &[choices::Core] = &[
                            choices::Core::Client_SendTo_Proxy_Request,
                            choices::Core::Proxy_ReceivedFrom_Client_Request,
                            choices::Core::Proxy_SendTo_Remote_Request,
                            choices::Core::Remote_ReceivedFrom_Proxy_Request,
                            choices::Core::Remote_SendTo_Proxy_Response_ETag0,
                            choices::Core::Proxy_ReceivedFrom_Remote_Response_ETag0,
                            choices::Core::Proxy_Cache_Store,
                            choices::Core::Proxy_SendTo_Client_Response_ETag0,
                            choices::Core::Client_ReceivedFrom_Response_ETag0,
                          ];
                        }
                        pub mod core{
                          pub mod choices{ pub enum Core{
                            Client_SendTo_Proxy_Request,
                            Proxy_ReceivedFrom_Client_Request,
                            Proxy_Cache_Find_Returns_Some,
                            Proxy_SendTo_Remote_Request_ETag0,
                            Remote_ReceivedFrom_Proxy_Request_ETag0,
                            Remote_SendTo_Proxy_Response_304,
                            Proxy_ReceivedFrom_Remote_Response_304,
                            Proxy_ReceivedFrom_Remote_Process_Is304Transformation,
                            Proxy_SendTo_Client_Response_ETag0,
                            Client_ReceivedFrom_Proxy_Response_ETag0,
                          }}
                          pub const core : &[choices::Core] = &[
                            choices::Core::Client_SendTo_Proxy_Request,
                            choices::Core::Proxy_ReceivedFrom_Client_Request,
                            choices::Core::Proxy_Cache_Find_Returns_Some,
                            choices::Core::Proxy_SendTo_Remote_Request_ETag0,
                            choices::Core::Remote_ReceivedFrom_Proxy_Request_ETag0,
                            choices::Core::Remote_SendTo_Proxy_Response_304,
                            choices::Core::Proxy_ReceivedFrom_Remote_Response_304,
                            choices::Core::Proxy_Cache_Find_Returns_Some,
                            choices::Core::Proxy_ReceivedFrom_Remote_Process_Is304Transformation,
                            choices::Core::Proxy_SendTo_Client_Response_ETag0,
                            choices::Core::Client_ReceivedFrom_Proxy_Response_ETag0,
                          ];
                        }
                      }
                      pub mod events{
                        mod subs{
                          pub mod oneset{
                            pub fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::super::super::subs::oneset::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                              p.append(&mut match v{
                                super::super::super::subs::oneset::choices::Core::Client_SendTo_Proxy_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Send;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Test(Client(BetweenProxy(Send(request()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_ReceivedFrom_Client_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Receive;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Core(Process(BetweenClient(Receive(request()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_SendTo_Remote_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Send;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Core(Process(BetweenRemote(Send(request()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Remote_ReceivedFrom_Proxy_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Receive;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Test(Remote(BetweenProxy(Receive(request()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Remote_SendTo_Proxy_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Test(Remote(BetweenProxy(Send(response_etag0()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_ReceivedFrom_Remote_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Start;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(BetweenRemote(Receive(Start(response_etag0())))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_Cache_Store=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Store;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(Cache(Store(response_etag0()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_SendTo_Client_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(BetweenClient(Send(response_etag0()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Client_ReceivedFrom_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Receive;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Test(Client(BetweenProxy(Receive(response_etag0()))))]
                                },
                              });
                              p
                            }
                          }
                          pub mod core{
                            pub fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::super::super::subs::core::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                              p.append(&mut match v{
                                super::super::super::subs::core::choices::Core::Client_SendTo_Proxy_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Send;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Test(Client(BetweenProxy(Send(request()))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_ReceivedFrom_Client_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Receive;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Core(Process(BetweenClient(Receive(request()))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_Cache_Find_Returns_Some=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Find;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::find::Core::Returns;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(Cache(Find(Returns(Some(response_etag0()))))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_SendTo_Remote_Request_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::request_etag0;
                                  vec![Core(Process(BetweenRemote(Send(request_etag0()))))]
                                },
                                super::super::super::subs::core::choices::Core::Remote_ReceivedFrom_Proxy_Request_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Receive;
                                  use crate::test::testers::specifics::proxy::etag::common::request_etag0;
                                  vec![Test(Remote(BetweenProxy(Receive(request_etag0()))))]
                                },
                                super::super::super::subs::core::choices::Core::Remote_SendTo_Proxy_Response_304=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::response_304;
                                  vec![Test(Remote(BetweenProxy(Send(response_304()))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_ReceivedFrom_Remote_Response_304=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Start;
                                  use crate::test::testers::specifics::proxy::etag::common::response_304;
                                  vec![Core(Process(BetweenRemote(Receive(Start(response_304())))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_ReceivedFrom_Remote_Process_Is304Transformation=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Process as ReceiveProcess;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::process::Core::Is304Transformation;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::process::is_304_transformation::Core::Returned;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(BetweenRemote(Receive(ReceiveProcess(Is304Transformation(Returned(response_etag0())))))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_SendTo_Client_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(BetweenClient(Send(response_etag0()))))]
                                },
                                super::super::super::subs::core::choices::Core::Client_ReceivedFrom_Proxy_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Receive;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Test(Client(BetweenProxy(Receive(response_etag0()))))]
                                },
                              });
                              p
                            }
                          }
                        }
                        fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                          p.append(&mut match v{
                            super::choices::Core::OneSet=>super::subs::oneset::core.iter().map(|v| v.clone()).fold(vec![], subs::oneset::fold),
                            super::choices::Core::Core=>super::subs::core::core.iter().map(|v| v.clone()).fold(vec![], subs::core::fold),
                          });
                          p
                        }
                        pub fn core()->Vec<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                          super::core.iter().fold(vec![], fold)
                        }
                        pub fn function(panic_handler: fn(), sut: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>){
                          crate::test::common::util::events::compare::core(panic_handler, core(), sut);
                        }
                      }
                      mod choices{ pub enum Core{
                        OneSet,
                        Core
                      }}
                      pub const core : &[choices::Core] = &[
                        choices::Core::OneSet,
                        choices::Core::Core,
                      ];
                    }
                    pub mod choices{ pub enum Core{
                      Act_OneSet,
                      Act_Core,
                      Assert(fn()->())
                    }}
                    pub const core : &[choices::Core] = &[
                      choices::Core::Act_OneSet,
                      choices::Core::Act_Core,
                      choices::Core::Assert(|| panic!()),
                    ];
                  }
                  pub struct Runner<TState: crate::test::common::sut::abstracts::business::test::Core>{
                    pub state_new: fn()->TState,
                    pub state_get_log: fn(&TState)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>
                  }
                  impl<TState: crate::test::common::sut::abstracts::business::test::Core> Runner<TState>{
                    pub fn run(self){
                      steps::core.iter().fold(
                        (self.state_new)(),
                        |p, v|{
                          match v{
                            steps::choices::Core::Act_OneSet=>{
                              let ret = (p,);
                              let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(
                                ret.0,
                                crate::test::testers::specifics::proxy::etag::common::response_etag0()
                              );
                              let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(
                                ret.0,
                                crate::test::testers::common::http::cern::request()
                              );
                              let ret = ret.0;
                              ret
                            },
                            steps::choices::Core::Act_Core=>{
                              let ret = (p,);
                              let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(
                                ret.0,
                                crate::test::testers::specifics::proxy::etag::common::response_304()
                              );
                              let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(
                                ret.0,
                                crate::test::testers::common::http::cern::request()
                              );
                              let ret = ret.0;
                              ret
                            },
                            steps::choices::Core::Assert(panic_handler)=>{ steps::assert::events::function(*panic_handler, (self.state_get_log)(&p)) ; p },
                          }
                        }
                      );
                    }
                  }
                }
              }
              pub mod implmnts{
                pub mod test{ pub mod unit{
                  pub fn run(){
                    super::super::super::abstracts::core::Runner{
                      state_new: || crate::test::common::sut::implmnts::core::test::unit::core::new(), 
                      state_get_log: |p|{
                        let ret = p;
                        let ret = &ret.item;
                        let ret = ret.log.clone();
                        ret
                      }
                    }.run()
                  }
                }}
              }
            }
            pub mod modified{
              mod abstracts{
                pub mod core{
                  mod steps{
                    pub mod assert{
                      mod subs{
                        pub mod oneset{
                          pub mod choices{ pub enum Core{
                            Client_SendTo_Proxy_Request,
                            Proxy_ReceivedFrom_Client_Request,
                            Proxy_SendTo_Remote_Request,
                            Remote_ReceivedFrom_Proxy_Request,
                            Remote_SendTo_Proxy_Response_ETag0,
                            Proxy_ReceivedFrom_Remote_Response_ETag0,
                            Proxy_Cache_Store,
                            Proxy_SendTo_Client_Response_ETag0,
                            Client_ReceivedFrom_Response_ETag0,
                          }}
                          pub const core : &[choices::Core] = &[
                            choices::Core::Client_SendTo_Proxy_Request,
                            choices::Core::Proxy_ReceivedFrom_Client_Request,
                            choices::Core::Proxy_SendTo_Remote_Request,
                            choices::Core::Remote_ReceivedFrom_Proxy_Request,
                            choices::Core::Remote_SendTo_Proxy_Response_ETag0,
                            choices::Core::Proxy_ReceivedFrom_Remote_Response_ETag0,
                            choices::Core::Proxy_Cache_Store,
                            choices::Core::Proxy_SendTo_Client_Response_ETag0,
                            choices::Core::Client_ReceivedFrom_Response_ETag0,
                          ];
                        }
                        pub mod core{
                          pub mod choices{ pub enum Core{
                            Client_SendTo_Proxy_Request,
                            Proxy_ReceivedFrom_Client_Request,
                            Proxy_Cache_Find_Returns_Some,
                            Proxy_SendTo_Remote_Request_ETag0,
                            Remote_ReceivedFrom_Proxy_Request_ETag0,
                            Remote_SendTo_Proxy_Response_ETag1,
                            Proxy_ReceivedFrom_Remote_Response_ETag1,
                            Proxy_Cache_Store,
                            Proxy_SendTo_Client_Response_ETag1,
                            Client_ReceivedFrom_Proxy_Response_ETag1,
                          }}
                          pub const core : &[choices::Core] = &[
                            choices::Core::Client_SendTo_Proxy_Request,
                            choices::Core::Proxy_ReceivedFrom_Client_Request,
                            choices::Core::Proxy_Cache_Find_Returns_Some,
                            choices::Core::Proxy_SendTo_Remote_Request_ETag0,
                            choices::Core::Remote_ReceivedFrom_Proxy_Request_ETag0,
                            choices::Core::Remote_SendTo_Proxy_Response_ETag1,
                            choices::Core::Proxy_ReceivedFrom_Remote_Response_ETag1,
                            choices::Core::Proxy_Cache_Store,
                            choices::Core::Proxy_SendTo_Client_Response_ETag1,
                            choices::Core::Client_ReceivedFrom_Proxy_Response_ETag1,
                          ];
                        }
                      }
                      pub mod events{
                        mod subs{
                          pub mod oneset{
                            pub fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::super::super::subs::oneset::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                              p.append(&mut match v{
                                super::super::super::subs::oneset::choices::Core::Client_SendTo_Proxy_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Send;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Test(Client(BetweenProxy(Send(request()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_ReceivedFrom_Client_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Receive;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Core(Process(BetweenClient(Receive(request()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_SendTo_Remote_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Send;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Core(Process(BetweenRemote(Send(request()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Remote_ReceivedFrom_Proxy_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Receive;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Test(Remote(BetweenProxy(Receive(request()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Remote_SendTo_Proxy_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Test(Remote(BetweenProxy(Send(response_etag0()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_ReceivedFrom_Remote_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Start;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(BetweenRemote(Receive(Start(response_etag0())))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_Cache_Store=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Store;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(Cache(Store(response_etag0()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Proxy_SendTo_Client_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(BetweenClient(Send(response_etag0()))))]
                                },
                                super::super::super::subs::oneset::choices::Core::Client_ReceivedFrom_Response_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Receive;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Test(Client(BetweenProxy(Receive(response_etag0()))))]
                                },
                              });
                              p
                            }
                          }
                          pub mod core{
                            pub fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::super::super::subs::core::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                              p.append(&mut match v{
                                super::super::super::subs::core::choices::Core::Client_SendTo_Proxy_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Send;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Test(Client(BetweenProxy(Send(request()))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_ReceivedFrom_Client_Request=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Receive;
                                  use crate::test::testers::common::http::cern::request;
                                  vec![Core(Process(BetweenClient(Receive(request()))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_Cache_Find_Returns_Some=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Find;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::find::Core::Returns;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                  vec![Core(Process(Cache(Find(Returns(Some(response_etag0()))))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_SendTo_Remote_Request_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::request_etag0;
                                  vec![Core(Process(BetweenRemote(Send(request_etag0()))))]
                                },
                                super::super::super::subs::core::choices::Core::Remote_ReceivedFrom_Proxy_Request_ETag0=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Receive;
                                  use crate::test::testers::specifics::proxy::etag::common::request_etag0;
                                  vec![Test(Remote(BetweenProxy(Receive(request_etag0()))))]
                                },
                                super::super::super::subs::core::choices::Core::Remote_SendTo_Proxy_Response_ETag1=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag1;
                                  vec![Test(Remote(BetweenProxy(Send(response_etag1()))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_ReceivedFrom_Remote_Response_ETag1=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Start;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag1;
                                  vec![Core(Process(BetweenRemote(Receive(Start(response_etag1())))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_Cache_Store=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Store;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag1;
                                  vec![Core(Process(Cache(Store(response_etag1()))))]
                                },
                                super::super::super::subs::core::choices::Core::Proxy_SendTo_Client_Response_ETag1=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Send;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag1;
                                  vec![Core(Process(BetweenClient(Send(response_etag1()))))]
                                },
                                super::super::super::subs::core::choices::Core::Client_ReceivedFrom_Proxy_Response_ETag1=>{
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                  use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Receive;
                                  use crate::test::testers::specifics::proxy::etag::common::response_etag1;
                                  vec![Test(Client(BetweenProxy(Receive(response_etag1()))))]
                                },
                              });
                              p
                            }
                          }
                        }
                        fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                          p.append(&mut match v{
                            super::choices::Core::OneSet=>super::subs::oneset::core.iter().map(|v| v.clone()).fold(vec![], subs::oneset::fold),
                            super::choices::Core::Core=>super::subs::core::core.iter().map(|v| v.clone()).fold(vec![], subs::core::fold),
                          });
                          p
                        }
                        pub fn core()->Vec<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                          super::core.iter().fold(vec![], fold)
                        }
                        pub fn function(panic_handler: fn(), sut: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>){
                          crate::test::common::util::events::compare::core(panic_handler, core(), sut);
                        }
                      }
                      mod choices{ pub enum Core{
                        OneSet,
                        Core
                      }}
                      pub const core : &[choices::Core] = &[
                        choices::Core::OneSet,
                        choices::Core::Core,
                      ];
                    }
                    pub mod choices{ pub enum Core{
                      Act_OneSet,
                      Act_Core,
                      Assert(fn()->())
                    }}
                    pub const core : &[choices::Core] = &[
                      choices::Core::Act_OneSet,
                      choices::Core::Act_Core,
                      choices::Core::Assert(|| panic!()),
                    ];
                  }
                  pub struct Runner<TState: crate::test::common::sut::abstracts::business::test::Core>{
                    pub state_new: fn()->TState,
                    pub state_get_log: fn(&TState)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>
                  }
                  impl<TState: crate::test::common::sut::abstracts::business::test::Core> Runner<TState>{
                    pub fn run(self){
                      steps::core.iter().fold(
                        (self.state_new)(),
                        |p, v|{
                          match v{
                            steps::choices::Core::Act_OneSet=>{
                              let ret = (p,);
                              let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(
                                ret.0,
                                crate::test::testers::specifics::proxy::etag::common::response_etag0()
                              );
                              let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(
                                ret.0,
                                crate::test::testers::common::http::cern::request()
                              );
                              let ret = ret.0;
                              ret
                            },
                            steps::choices::Core::Act_Core=>{
                              let ret = (p,);
                              let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(
                                ret.0,
                                crate::test::testers::specifics::proxy::etag::common::response_etag1()
                              );
                              let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(
                                ret.0,
                                crate::test::testers::common::http::cern::request()
                              );
                              let ret = ret.0;
                              ret
                            },
                            steps::choices::Core::Assert(panic_handler)=>{ steps::assert::events::function(*panic_handler, (self.state_get_log)(&p)) ; p },
                          }
                        }
                      );
                    }
                  }
                }
              }
              pub mod implmnts{
                pub mod test{ pub mod unit{
                  pub fn run(){
                    super::super::super::abstracts::core::Runner{
                      state_new: || crate::test::common::sut::implmnts::core::test::unit::core::new(), 
                      state_get_log: |p|{
                        let ret = p;
                        let ret = &ret.item;
                        let ret = ret.log.clone();
                        ret
                      }
                    }.run()
                  }
                }}
              }
            }
          }
        }
        pub mod lmod{
          pub mod not_modified{
            mod abstracts{
              pub mod core{
                mod steps{
                  pub mod assert{
                    mod subs{
                      pub mod oneset{
                        pub mod choices{ pub enum Core{
                          Client_SendTo_Proxy_Request,
                          Proxy_ReceivedFrom_Client_Request,
                          Proxy_SendTo_Remote_Request,
                          Remote_ReceivedFrom_Proxy_Request,
                          Remote_SendTo_Proxy_Response_LMod0,
                          Proxy_ReceivedFrom_Remote_Response_LMod0,
                          Proxy_Cache_Store,
                          Proxy_SendTo_Client_Response_LMod0,
                          Client_ReceivedFrom_Response_LMod0,
                        }}
                        pub const core : &[choices::Core] = &[
                          choices::Core::Client_SendTo_Proxy_Request,
                          choices::Core::Proxy_ReceivedFrom_Client_Request,
                          choices::Core::Proxy_SendTo_Remote_Request,
                          choices::Core::Remote_ReceivedFrom_Proxy_Request,
                          choices::Core::Remote_SendTo_Proxy_Response_LMod0,
                          choices::Core::Proxy_ReceivedFrom_Remote_Response_LMod0,
                          choices::Core::Proxy_Cache_Store,
                          choices::Core::Proxy_SendTo_Client_Response_LMod0,
                          choices::Core::Client_ReceivedFrom_Response_LMod0,
                        ];
                      }
                      pub mod core{
                        pub mod choices{ pub enum Core{
                          Client_SendTo_Proxy_Request,
                          Proxy_ReceivedFrom_Client_Request,
                          Proxy_Cache_Find_Returns_Some,
                          Proxy_SendTo_Remote_Request_LMod0,
                          Remote_ReceivedFrom_Proxy_Request_LMod0,
                          Remote_SendTo_Proxy_Response_304,
                          Proxy_ReceivedFrom_Remote_Response_304,
                          Proxy_ReceivedFrom_Remote_Process_Is304Transformation,
                          Proxy_SendTo_Client_Response_LMod0,
                          Client_ReceivedFrom_Proxy_Response_LMod0,
                        }}
                        pub const core : &[choices::Core] = &[
                          choices::Core::Client_SendTo_Proxy_Request,
                          choices::Core::Proxy_ReceivedFrom_Client_Request,
                          choices::Core::Proxy_Cache_Find_Returns_Some,
                          choices::Core::Proxy_SendTo_Remote_Request_LMod0,
                          choices::Core::Remote_ReceivedFrom_Proxy_Request_LMod0,
                          choices::Core::Remote_SendTo_Proxy_Response_304,
                          choices::Core::Proxy_ReceivedFrom_Remote_Response_304,
                          choices::Core::Proxy_Cache_Find_Returns_Some,
                          choices::Core::Proxy_ReceivedFrom_Remote_Process_Is304Transformation,
                          choices::Core::Proxy_SendTo_Client_Response_LMod0,
                          choices::Core::Client_ReceivedFrom_Proxy_Response_LMod0,
                        ];
                      }
                    }
                    pub mod events{
                      mod subs{
                        pub mod oneset{
                          pub fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::super::super::subs::oneset::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                            p.append(&mut match v{
                              super::super::super::subs::oneset::choices::Core::Client_SendTo_Proxy_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Send;
                                use crate::test::testers::common::http::cern::request;
                                vec![Test(Client(BetweenProxy(Send(request()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_ReceivedFrom_Client_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Receive;
                                use crate::test::testers::common::http::cern::request;
                                vec![Core(Process(BetweenClient(Receive(request()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_SendTo_Remote_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Send;
                                use crate::test::testers::common::http::cern::request;
                                vec![Core(Process(BetweenRemote(Send(request()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Remote_ReceivedFrom_Proxy_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Receive;
                                use crate::test::testers::common::http::cern::request;
                                vec![Test(Remote(BetweenProxy(Receive(request()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Remote_SendTo_Proxy_Response_ETag0=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Send;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Test(Remote(BetweenProxy(Send(response_etag0()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_ReceivedFrom_Remote_Response_ETag0=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Start;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Core(Process(BetweenRemote(Receive(Start(response_etag0())))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_Cache_Store=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Store;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Core(Process(Cache(Store(response_etag0()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Proxy_SendTo_Client_Response_ETag0=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Send;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Core(Process(BetweenClient(Send(response_etag0()))))]
                              },
                              super::super::super::subs::oneset::choices::Core::Client_ReceivedFrom_Response_ETag0=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Receive;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Test(Client(BetweenProxy(Receive(response_etag0()))))]
                              },
                            });
                            p
                          }
                        }
                        pub mod core{
                          pub fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::super::super::subs::core::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                            p.append(&mut match v{
                              super::super::super::subs::core::choices::Core::Client_SendTo_Proxy_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Send;
                                use crate::test::testers::common::http::cern::request;
                                vec![Test(Client(BetweenProxy(Send(request()))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_ReceivedFrom_Client_Request=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Receive;
                                use crate::test::testers::common::http::cern::request;
                                vec![Core(Process(BetweenClient(Receive(request()))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_Cache_Find_Returns_Some=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::Cache;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::Core::Find;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::cache::find::Core::Returns;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Core(Process(Cache(Find(Returns(Some(response_etag0()))))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_SendTo_Remote_Request_ETag0=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Send;
                                use crate::test::testers::specifics::proxy::etag::common::request_etag0;
                                vec![Core(Process(BetweenRemote(Send(request_etag0()))))]
                              },
                              super::super::super::subs::core::choices::Core::Remote_ReceivedFrom_Proxy_Request_ETag0=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Receive;
                                use crate::test::testers::specifics::proxy::etag::common::request_etag0;
                                vec![Test(Remote(BetweenProxy(Receive(request_etag0()))))]
                              },
                              super::super::super::subs::core::choices::Core::Remote_SendTo_Proxy_Response_304=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Remote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::remote::between_proxy::Core::Send;
                                use crate::test::testers::specifics::proxy::etag::common::response_304;
                                vec![Test(Remote(BetweenProxy(Send(response_304()))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_ReceivedFrom_Remote_Response_304=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Start;
                                use crate::test::testers::specifics::proxy::etag::common::response_304;
                                vec![Core(Process(BetweenRemote(Receive(Start(response_304())))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_ReceivedFrom_Remote_Process_Is304Transformation=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenRemote;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::Core::Receive;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::Core::Process as ReceiveProcess;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::process::Core::Is304Transformation;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_remote::receive::process::is_304_transformation::Core::Returned;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Core(Process(BetweenRemote(Receive(ReceiveProcess(Is304Transformation(Returned(response_etag0())))))))]
                              },
                              super::super::super::subs::core::choices::Core::Proxy_SendTo_Client_Response_ETag0=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Core;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::Core::Process;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::Core::BetweenClient;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::core::process::between_client::Core::Send;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Core(Process(BetweenClient(Send(response_etag0()))))]
                              },
                              super::super::super::subs::core::choices::Core::Client_ReceivedFrom_Proxy_Response_ETag0=>{
                                use crate::test::common::sut::implmnts::log::test::unit::choices::Core::Test;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::Core::Client;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::Core::BetweenProxy;
                                use crate::test::common::sut::implmnts::log::test::unit::choices::test::client::between_proxy::Core::Receive;
                                use crate::test::testers::specifics::proxy::etag::common::response_etag0;
                                vec![Test(Client(BetweenProxy(Receive(response_etag0()))))]
                              },
                            });
                            p
                          }
                        }
                      }
                      fn fold(mut p: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>, v: &super::choices::Core)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                        p.append(&mut match v{
                          super::choices::Core::OneSet=>super::subs::oneset::core.iter().map(|v| v.clone()).fold(vec![], subs::oneset::fold),
                          super::choices::Core::Core=>super::subs::core::core.iter().map(|v| v.clone()).fold(vec![], subs::core::fold),
                        });
                        p
                      }
                      pub fn core()->Vec<crate::test::common::sut::implmnts::log::test::unit::choices::Core>{
                        super::core.iter().fold(vec![], fold)
                      }
                      pub fn function(panic_handler: fn(), sut: Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>){
                        crate::test::common::util::events::compare::core(panic_handler, core(), sut);
                      }
                    }
                    mod choices{ pub enum Core{
                      OneSet,
                      Core
                    }}
                    pub const core : &[choices::Core] = &[
                      choices::Core::OneSet,
                      choices::Core::Core,
                    ];
                  }
                  pub mod choices{ pub enum Core{
                    Act_OneSet,
                    Act_Core,
                    Assert(fn()->())
                  }}
                  pub const core : &[choices::Core] = &[
                    choices::Core::Act_OneSet,
                    choices::Core::Act_Core,
                    choices::Core::Assert(|| panic!()),
                  ];
                }
                pub struct Runner<TState: crate::test::common::sut::abstracts::business::test::Core>{
                  pub state_new: fn()->TState,
                  pub state_get_log: fn(&TState)->Vec::<crate::test::common::sut::implmnts::log::test::unit::choices::Core>
                }
                impl<TState: crate::test::common::sut::abstracts::business::test::Core> Runner<TState>{
                  pub fn run(self){
                    steps::core.iter().fold(
                      (self.state_new)(),
                      |p, v|{
                        match v{
                          steps::choices::Core::Act_OneSet=>{
                            let ret = (p,);
                            let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(
                              ret.0,
                              crate::test::testers::specifics::proxy::etag::common::response_lmod0()
                            );
                            let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(
                              ret.0,
                              crate::test::testers::common::http::cern::request()
                            );
                            let ret = ret.0;
                            ret
                          },
                          steps::choices::Core::Act_Core=>{
                            let ret = (p,);
                            let ret = crate::test::common::sut::abstracts::business::test::subs::process::remote::between_proxy::cans::set::Core::run(
                              ret.0,
                              crate::test::testers::specifics::proxy::etag::common::response_304()
                            );
                            let ret = crate::test::common::sut::abstracts::business::test::subs::process::client::between_proxy::send::Core::run(
                              ret.0,
                              crate::test::testers::common::http::cern::request()
                            );
                            let ret = ret.0;
                            ret
                          },
                          steps::choices::Core::Assert(panic_handler)=>{ steps::assert::events::function(*panic_handler, (self.state_get_log)(&p)) ; p },
                        }
                      }
                    );
                  }
                }
              }
            }
            pub mod implmnts{
              pub mod test{ pub mod unit{
                pub fn run(){
                  super::super::super::abstracts::core::Runner{
                    state_new: || crate::test::common::sut::implmnts::core::test::unit::core::new(), 
                    state_get_log: |p|{
                      let ret = p;
                      let ret = &ret.item;
                      let ret = ret.log.clone();
                      ret
                    }
                  }.run()
                }
              }}
            }
          }
        }
      }
    }
  }
  mod suites{
    mod test{ mod unit{
      mod plain{ mod strictly_no_renew{
        #[test]
        fn test(){
          crate::test::testers::specifics::proxy::plain::strictly_no_renew::implmnts::test::unit::run();
        }
      }}
      mod etag{ 
        mod not_modified{
          #[test]
          fn test(){
            crate::test::testers::specifics::proxy::etag::specifics::not_modified::implmnts::test::unit::run();
          }
        }
        mod modified{
          #[test] 
          fn test(){
            crate::test::testers::specifics::proxy::etag::specifics::modified::implmnts::test::unit::run();
          }
        }
      }
      mod lmod{
        mod not_modified{
          #[test]
          fn test(){
            crate::test::testers::specifics::proxy::lmod::specifics::not_modified::implmnts::test::unit::run();
          }
        }
      }
    }}
  }
}

fn main(){}