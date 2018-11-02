extern crate servo_media;

use servo_media::audio::node::{AudioNodeInit, AudioNodeMessage, AudioScheduledSourceNodeMessage};
use servo_media::audio::oscillator_node::OscillatorNodeOptions;
use servo_media::audio::oscillator_node::OscillatorType::Triangle;
use servo_media::audio::param::{ParamType, RampKind, UserAutomationEvent};
use servo_media::ServoMedia;
use std::sync::Arc;
use std::{thread, time};

fn run_example(servo_media: Arc<ServoMedia>) {
    let context = servo_media.create_audio_context(Default::default());
    let dest = context.dest_node();
    let mut options = OscillatorNodeOptions::default();
    options.freq = 150.;
    //options.oscillator_type = Sawtooth; 
    let osc1 = context.create_node(AudioNodeInit::OscillatorNode(options), Default::default());
    options.oscillator_type = Triangle; 
    //options.freq = 800.;
    let osc2 = context.create_node(AudioNodeInit::OscillatorNode(options), Default::default()); 
    context.connect_ports(osc1.output(0), dest.input(0));
    
    let _ = context.resume();
    context.message_node(
        osc1,
        AudioNodeMessage::AudioScheduledSourceNode(AudioScheduledSourceNodeMessage::Start(0.)),
    );
    
    thread::sleep(time::Duration::from_millis(3000));
    context.connect_ports(osc2.output(0), dest.input(0));
    context.message_node(
        osc2,
        AudioNodeMessage::AudioScheduledSourceNode(AudioScheduledSourceNodeMessage::Start(0.)),
    );
    thread::sleep(time::Duration::from_millis(3000));
}

fn main() {
    if let Ok(servo_media) = ServoMedia::get() {
        run_example(servo_media);
    } else {
        unreachable!();
    }
}
