// Copyright 2015 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use device::{Device, Factory, Resources};
use extra::stream::Stream;
use render::{Renderer, RenderFactory};
use render::target::Output;


/// Generic output window.
pub trait Window<R: Resources>: Output<R> {
    /// Swap front and back buffers.
    fn swap_buffers(&mut self);
}

/// A canvas with everything you need to draw on it.
pub struct Canvas<W, D: Device, F> {
    /// Output window.
    pub output: W,
    /// Graphics device.
    pub device: D,
    /// Resource factory.
    pub factory: F,
    /// Renderer front-end.
    pub renderer: Renderer<D::Resources, D::CommandBuffer>,
}

/// Something that can be transformed into `Canvas`.
pub trait IntoCanvas<W, D: Device, F> {
    /// Transform into `Canvas`.
    fn into_canvas(self) -> Canvas<W, D, F>;
}

impl<W, D: Device, F: Factory<D::Resources>> IntoCanvas<W, D, F> for (W, D, F) {
    fn into_canvas(mut self) -> Canvas<W, D, F> {
        let renderer = self.2.create_renderer();
        Canvas {
            output: self.0,
            device: self.1,
            factory: self.2,
            renderer: renderer,
        }
    }
}

impl<D: Device, F: Factory<D::Resources>, O: Output<D::Resources>>
Stream<D::Resources> for Canvas<O, D, F> {
    type CommandBuffer = D::CommandBuffer;
    type Output = O;

    fn get_output(&self) -> &O {
        &self.output
    }

    fn access(&mut self) -> (&mut Renderer<D::Resources, D::CommandBuffer>, &O) {
        (&mut self.renderer, &self.output)
    }
}

impl<D: Device, F: Factory<D::Resources>, W: Window<D::Resources>> Canvas<W, D, F> {
    /// Show what we've been drawing all this time.
    pub fn present(&mut self) {
        self.device.submit(self.renderer.as_buffer());
        self.output.swap_buffers();
        self.device.after_frame();
        self.factory.cleanup();
        self.renderer.reset();
    }
}