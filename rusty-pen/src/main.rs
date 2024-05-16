use vector::Vector;
use speedy2d::color::Color;
use speedy2d::window::{self, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let window = Window::new_centered("Pen", (800,400)).unwrap();

    let win = MyWindowHandler {
        pen: Pen::new(400.0, 0.0, 200.0),
        pen2: Pen::new(400.0, 0.0, 400.0)
    };

    window.run_loop(win);
}

struct MyWindowHandler {
    pen: Pen,
    pen2: Pen,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        // Limpa a tela
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        // Atualiza o pendulo
        self.pen.update();

        // Desenha o pendulo
        self.pen.draw(graphics);

        self.pen2.update();

        // Desenha o pendulo
        self.pen2.draw(graphics);

        // Redesenha a tela
        helper.request_redraw();
    }
}

struct Pen {
    origin: Vector, // origem do pendulo
    position: Vector, // posicao do pendulo

    angle: f32, // angulo do pendulo

    angular_velocity: f32, // velocidade angular do pendulo
    angular_acceleration: f32, // aceleracao angular do pendulo

    r: f32, // langura do pendulo
    m: f32, // massa do pendulo
    g: f32, // aceleracao da gravidade
}

impl Pen {
    fn new(x: f32, y: f32, r: f32) -> Pen{
        Pen {
            origin: Vector::new(x, y), 
            position: Vector::new(x, y), 
            angle: 1.0, 
            angular_velocity: 0.0, 
            angular_acceleration: 0.0, 
            r: r, 
            m: 5.0, 
            g: 2.9,
        }
    }

    fn update(&mut self){
        // calcula a aceleracao angular
        self.angular_acceleration = -1.0 * self.g / self.r * self.angle.sin() / self.r;

        // calcula a velocidade angular
        self.angular_velocity  += self.angular_acceleration;

        // calcula o angulo
        self.angle += self.angular_velocity;


        self.position.set(self.r * self.angle.sin(), self.r * self.angle.cos());

        self.position.add(&self.origin);
    }   

    fn draw(&self, graphics: &mut Graphics2D){
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y), 
            3.0, 
            Color::RED);

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        // funcao assosiada construtora da struct Vector 
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y } // return Vector
        }
        // funcao que retorna a soma de dois vetores
        pub fn add(&mut self, other: &Vector) -> &Vector{
            self.x += other.x;
            self.y += other.y;
            self
        }

        // funcao que seta os vetores
        pub fn set(&mut self,   x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}
