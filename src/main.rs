use piston_window::*;
use rand::*;

const HEIGHT:f64 = 720.0;
const WIDTH:f64 = 1280.0;

struct Bubble {
    speed:f64,
    x:f64, //x position
    y:f64, //y position
    r:f64, //radius
}

impl Bubble {
    pub fn new(num:Option<f64>) -> Bubble{
        let r = (random::<f64>()*(WIDTH/8.0))+5.0;
        let mut b:Bubble = Bubble{
            speed: (random::<f64>()*90.0)+50.0,
            y: random::<f64>()*(HEIGHT+r),
            x:random::<f64>()*WIDTH,
            r:r
        };

        if let Some(y)=num{ //option is passed to make the num "new" function accept None as an argument 
            println!("{}",y);
            b.speed = 0.0;
            b.y=y;
        }
        return b
    }

    fn get_bubbles() -> Vec<Bubble>{
        let mut bubbles = Vec::new();
        let n = (random::<u32>()%15)+10;
        for _ in 0..n{
            bubbles.push(Bubble::new(Some(HEIGHT)));
            bubbles.push(Bubble::new(Some(0.0)));
            bubbles.push(Bubble::new(None));
        }
        bubbles
     
    }
}


fn main() {
    let bub = [1.0,97.0/255.0,0.0,1.0];
    let bg = [104.0/255.0,221.0/255.0,19.0/255.0,1.0];

    let mut bubbles:Vec<Bubble> = Bubble::get_bubbles();
    let mut window:PistonWindow = WindowSettings::new("LAVA LAMP",[WIDTH,HEIGHT]).exit_on_esc(true).build().unwrap();
    let mut events = window.events;

    while let Some(e) = events.next(&mut window){ //some new event "e" which occurs in the piston window
        if let Some(_) = e.render_args(){ //if the event "e" is render event ie create the first sketch in the window
            let bubbs = &bubbles;
            window.draw_2d(&e, |c,g,_|{
                clear(bg,g); //g: graphics
                for b in bubbs{
                    ellipse(bub, [b.x-b.r,b.y-b.r,b.r*2.0,b.r*2.0], c.transform,g);
                }
            });()
        } 

        if let Some(u) = e.update_args(){ //if the event "e" is updating the stuff in the window
            let bubbs = &mut bubbles; //passing reference to make sure the variables scope is not lost outside the if condition
            for b in bubbs{
                b.y -= b.speed*u.dt; //u.dt is updates events delta time
                if b.y+b.r <= 0.0 {b.y = HEIGHT +b.r}
            }
        } 

    }


}
