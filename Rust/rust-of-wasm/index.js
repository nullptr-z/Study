const rust = import('./pkg')

rust.then(r => r.greet("my name is Zheng")).catch(console.error);
