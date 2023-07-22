enum DinoState {
    RUNNING,
    JUMPING
}

struct Dino {
    state: DinoState,
    position: (f32, f32),
    velocity: f32        
}

impl Dino {

    fn run(self, delta: f32) {
        self.position.0 += self.velocity * delta; 
    }

}

