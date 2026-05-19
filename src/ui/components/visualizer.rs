use std::time::Instant;

#[derive(Debug)]
pub struct AudioVisualizer {
    bars: Vec<f32>,
    last_update: Instant,
    speed: f32,
}

impl AudioVisualizer {
    pub fn new(num_bars: usize) -> Self {
        Self {
            bars: vec![0.0; num_bars],
            last_update: Instant::now(),
            speed: 0.15,
        }
    }

    pub fn update(&mut self, is_playing: bool) {
        let elapsed = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();

        if is_playing {
            for (i, bar) in self.bars.iter_mut().enumerate() {
                let freq = (i as f32 * 0.7 + elapsed * 3.0).sin();
                let freq2 = (i as f32 * 1.3 + elapsed * 5.0).cos();
                let target = (freq * 0.5 + freq2 * 0.3 + 0.8).clamp(0.1, 1.0);
                *bar += (target - *bar) * self.speed;
            }
        } else {
            for bar in self.bars.iter_mut() {
                *bar *= 0.9;
                if *bar < 0.01 {
                    *bar = 0.0;
                }
            }
        }
    }

    pub fn render(&self, width: usize, height: usize) -> Vec<String> {
        let chars = ['░', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        let num_bars = self.bars.len();
        let _bar_width = width / num_bars;

        let mut lines = vec![String::new(); height];

        for (i, &value) in self.bars.iter().enumerate() {
            let bar_height = (value * height as f32).round() as usize;
            let char_idx = ((bar_height as f32 / height as f32) * (chars.len() - 1) as f32).round() as usize;
            let ch = chars[char_idx.min(chars.len() - 1)];

            for y in 0..height {
                let line_idx = height - 1 - y;
                if y < bar_height {
                    lines[line_idx].push(ch);
                } else {
                    lines[line_idx].push(' ');
                }
                if i < num_bars - 1 {
                    lines[line_idx].push(' ');
                }
            }
        }

        lines
    }
}
