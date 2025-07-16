//! # Bach: A Musical and Mathematical Composition Engine
//!
//! This module provides a comprehensive framework for musical representation, analysis,
//! and algorithmic composition, inspired by the structural and mathematical genius of
//! Johann Sebastian Bach. It defines a rich set of tools for working with musical
//! concepts such as notes, scales, chords, harmony, and counterpoint.
//!
//! ## Core Philosophy
//!
//! The `bach` module treats music not just as an art form, but as a mathematical
//! language. It provides the building blocks to explore the intricate relationships
_between music theory, harmony, and mathematical principles. The central `Bach`
//! trait encapsulates a wide array of functionalities, from basic note manipulation
//! to the generation of complex forms like fugues and chorales.
//!
//! ## Key Components
//!
//! - **Musical Primitives**: Enums like `Note`, `Interval`, `Scale`, and `ChordType`
//!   provide the fundamental vocabulary for musical expression.
//! - **Structural Elements**: Structs like `Voice`, `Chord`, and `Progression`
//!   represent the building blocks of musical composition.
//! - **The `Bach` Trait**: A powerful interface that defines methods for:
//!   - **Musical Operations**: Transposition, scale generation, chord building.
//!   - **Harmony and Counterpoint**: Generating progressions and checking rules.
//!   - **Algorithmic Composition**: Creating fugues, canons, and preludes.
//!   - **Musical Analysis**: Calculating tension, finding motifs, and measuring entropy.
//! - **`BachComposer`**: A concrete implementation of the `Bach` trait that provides
//!   a ready-to-use engine for musical tasks.

use std::collections::HashMap;
use rand::seq::SliceRandom;

/// Represents the twelve standard musical notes in Western music theory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Note {
    /// C
    C,
    /// C-sharp or D-flat
    CSharp,
    /// D
    D,
    /// D-sharp or E-flat
    DSharp,
    /// E
    E,
    /// F
    F,
    /// F-sharp or G-flat
    FSharp,
    /// G
    G,
    /// G-sharp or A-flat
    GSharp,
    /// A
    A,
    /// A-sharp or B-flat
    ASharp,
    /// B
    B,
}

/// Represents the distance in semitones between two notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    /// 0 semitones
    Unison = 0,
    /// 1 semitone
    MinorSecond = 1,
    /// 2 semitones
    MajorSecond = 2,
    /// 3 semitones
    MinorThird = 3,
    /// 4 semitones
    MajorThird = 4,
    /// 5 semitones
    PerfectFourth = 5,
    /// 6 semitones
    Tritone = 6,
    /// 7 semitones
    PerfectFifth = 7,
    /// 8 semitones
    MinorSixth = 8,
    /// 9 semitones
    MajorSixth = 9,
    /// 10 semitones
    MinorSeventh = 10,
    /// 11 semitones
    MajorSeventh = 11,
    /// 12 semitones
    Octave = 12,
}

/// Represents a collection of notes defined by a specific pattern of intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    /// The major scale (W-W-H-W-W-W-H).
    Major,
    /// The natural minor scale (W-H-W-W-H-W-W).
    NaturalMinor,
    /// A minor scale with a raised seventh degree.
    HarmonicMinor,
    /// A minor scale with a raised sixth and seventh degree when ascending.
    MelodicMinor,
    /// A mode of the major scale with a characteristic major sixth.
    Dorian,
    /// A mode of the major scale with a characteristic minor second.
    Phrygian,
    /// A mode of the major scale with a characteristic augmented fourth.
    Lydian,
    /// A mode of the major scale with a characteristic minor seventh.
    Mixolydian,
    /// A mode of the major scale with a characteristic diminished fifth.
    Locrian,
    /// A scale containing all twelve pitches.
    Chromatic,
    /// A scale consisting of six notes separated by whole-tone steps.
    WholeTone,
    /// A five-note scale.
    Pentatonic,
}

/// Represents the quality and structure of a chord.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordType {
    /// A three-note chord with a major third and a perfect fifth.
    Major,
    /// A three-note chord with a minor third and a perfect fifth.
    Minor,
    /// A three-note chord with a minor third and a diminished fifth.
    Diminished,
    /// A three-note chord with a major third and an augmented fifth.
    Augmented,
    /// A four-note chord with a major triad and a major seventh.
    MajorSeventh,
    /// A four-note chord with a minor triad and a minor seventh.
    MinorSeventh,
    /// A four-note chord with a major triad and a minor seventh.
    DominantSeventh,
    /// A four-note chord with a diminished triad and a diminished seventh.
    DiminishedSeventh,
    /// A four-note chord with a diminished triad and a minor seventh.
    HalfDiminished,
    /// A chord where the third is replaced by a major second.
    SuspendedSecond,
    /// A chord where the third is replaced by a perfect fourth.
    SuspendedFourth,
}

/// Represents a single melodic line in a musical composition.
#[derive(Debug, Clone)]
pub struct Voice {
    /// The sequence of notes and their durations (in beats).
    pub notes: Vec<(Note, f64)>,
    /// The octave of the voice (e.g., 4 for the middle octave).
    pub octave: i32,
    /// The MIDI velocity of the notes (0-127).
    pub velocity: u8,
}

/// Represents a set of notes played simultaneously.
#[derive(Debug, Clone)]
pub struct Chord {
    /// The root note of the chord.
    pub root: Note,
    /// The type or quality of the chord (e.g., Major, Minor).
    pub chord_type: ChordType,
    /// The octave of the chord's root note.
    pub octave: i32,
    /// The duration of the chord in beats.
    pub duration: f64,
}

/// Represents a sequence of chords.
#[derive(Debug, Clone)]
pub struct Progression {
    /// The sequence of chords in the progression.
    pub chords: Vec<Chord>,
    /// The key of the progression.
    pub key: Note,
    /// The scale used for the progression.
    pub scale: Scale,
}

/// Defines the rules for generating and checking counterpoint.
#[derive(Debug, Clone)]
pub struct CounterpointRules {
    /// Whether parallel fifths are allowed.
    pub parallel_fifths_allowed: bool,
    /// Whether parallel octaves are allowed.
    pub parallel_octaves_allowed: bool,
    /// Whether dissonances must resolve correctly.
    pub dissonance_resolution: bool,
    /// Whether to enforce smooth voice leading.
    pub voice_leading: bool,
}

/// A comprehensive trait for musical composition, analysis, and generation.
pub trait Bach {
    // Basic musical operations
    /// Converts a `Note` and octave to its corresponding frequency in Hertz.
    fn note_to_frequency(&self, note: Note, octave: i32) -> f64;
    /// Converts a frequency in Hertz to the nearest `Note` and its octave.
    fn frequency_to_note(&self, frequency: f64) -> (Note, i32);
    /// Calculates the musical interval between two notes.
    fn interval_between(&self, note1: Note, note2: Note) -> Interval;
    /// Transposes a note by a given interval.
    fn transpose_note(&self, note: Note, interval: Interval) -> Note;
    
    // Scale operations
    /// Generates the notes of a scale for a given root note.
    fn generate_scale(&self, root: Note, scale: Scale) -> Vec<Note>;
    /// Determines the degree of a note within a given scale (e.g., 1 for tonic).
    fn scale_degree(&self, note: Note, root: Note, scale: Scale) -> Option<u8>;
    /// Checks if a note belongs to a given scale.
    fn is_in_scale(&self, note: Note, root: Note, scale: Scale) -> bool;
    
    // Chord operations
    /// Builds the notes of a chord from a root note and chord type.
    fn build_chord(&self, root: Note, chord_type: ChordType) -> Vec<Note>;
    /// Returns the standard symbol for a chord (e.g., "Cmaj7").
    fn chord_symbol(&self, chord: &Chord) -> String;
    /// Analyzes a set of notes to determine the most likely chord.
    fn analyze_chord(&self, notes: &[Note]) -> Option<Chord>;
    /// Generates all possible inversions of a chord.
    fn chord_inversions(&self, chord: &Chord) -> Vec<Vec<Note>>;
    
    // Harmony operations
    /// Generates a chord progression of a given length in a specific key and scale.
    fn generate_progression(&self, key: Note, scale: Scale, length: usize) -> Progression;
    /// Analyzes a chord progression and returns the Roman numeral analysis.
    fn analyze_progression(&self, progression: &Progression) -> Vec<String>;
    /// Generates smooth voice leading between two chords.
    fn voice_leading(&self, chord1: &Chord, chord2: &Chord) -> Vec<Voice>;
    
    // Counterpoint operations
    /// Generates a counterpoint melody for a given cantus firmus.
    fn generate_counterpoint(&self, cantus_firmus: &Voice, rules: &CounterpointRules) -> Voice;
    /// Checks two voices for violations of counterpoint rules.
    fn check_counterpoint_rules(&self, voice1: &Voice, voice2: &Voice, rules: &CounterpointRules) -> Vec<String>;
    /// Inverts a melody around a central note by a given interval.
    fn invert_melody(&self, voice: &Voice, interval: Interval) -> Voice;
    /// Reverses a melody, creating its retrograde.
    fn retrograde_melody(&self, voice: &Voice) -> Voice;
    
    // Algorithmic composition
    /// Generates a fugue subject in a given key and scale.
    fn generate_fugue_subject(&self, key: Note, scale: Scale) -> Voice;
    /// Generates the answer to a fugue subject (typically transposed).
    fn generate_fugue_answer(&self, subject: &Voice, key: Note) -> Voice;
    /// Generates a complete fugue exposition with a specified number of voices.
    fn generate_fugue_exposition(&self, subject: &Voice, key: Note, voices: usize) -> Vec<Voice>;
    /// Generates a canon from a subject with a given interval and delay.
    fn generate_canon(&self, subject: &Voice, interval: Interval, delay: f64) -> Vec<Voice>;
    
    // Mathematical music theory
    /// Calculates the harmonic series for a given fundamental frequency.
    fn calculate_harmonic_series(&self, fundamental: f64, partials: usize) -> Vec<f64>;
    /// Calculates the frequency of a note using just intonation ratios.
    fn calculate_just_intonation(&self, note: Note, octave: i32) -> f64;
    /// Calculates the frequency of a note using 12-tone equal temperament.
    fn calculate_equal_temperament(&self, note: Note, octave: i32) -> f64;
    /// Calculates the beat frequency between two sound waves.
    fn calculate_beat_frequency(&self, freq1: f64, freq2: f64) -> f64;
    
    // Rhythm and meter
    /// Generates a rhythmic pattern based on a meter and complexity level.
    fn generate_rhythm_pattern(&self, meter: (u8, u8), complexity: f64) -> Vec<f64>;
    /// Applies syncopation to a rhythmic pattern.
    fn syncopate_rhythm(&self, rhythm: &[f64], syncopation_level: f64) -> Vec<f64>;
    /// Combines two rhythms to create a polyrhythm.
    fn polyrhythm(&self, rhythm1: &[f64], rhythm2: &[f64]) -> Vec<(f64, f64)>;
    
    // Musical analysis
    /// Performs a statistical analysis of a melody.
    fn analyze_melody(&self, voice: &Voice) -> HashMap<String, f64>;
    /// Finds recurring melodic patterns (motifs) in a voice.
    fn find_motifs(&self, voice: &Voice, min_length: usize) -> Vec<Vec<(Note, f64)>>;
    /// Calculates the harmonic tension at each point in a progression.
    fn calculate_tension(&self, progression: &Progression) -> Vec<f64>;
    /// Calculates the strength of the final cadence in a progression.
    fn calculate_cadence_strength(&self, progression: &Progression) -> f64;
    
    // Bach-specific algorithms
    /// Generates a four-part chorale in the style of J.S. Bach from a hymn tune.
    fn generate_bach_style_chorale(&self, hymn_tune: &Voice) -> Vec<Voice>;
    /// Generates a prelude in the style of J.S. Bach.
    fn generate_bach_style_prelude(&self, key: Note, scale: Scale) -> Voice;
    /// Generates a complete fugue in the style of J.S. Bach.
    fn generate_bach_style_fugue(&self, subject: &Voice, key: Note) -> Vec<Voice>;
    /// Applies baroque-style ornamentation to a voice.
    fn apply_bach_ornamentation(&self, voice: &Voice, ornamentation_level: f64) -> Voice;
    
    // Mathematical transformations
    /// Applies a transformation matrix to the notes of a voice.
    fn apply_transformation_matrix(&self, voice: &Voice, matrix: &[[f64; 4]; 4]) -> Voice;
    /// Generates a twelve-tone serial row.
    fn generate_serial_row(&self, length: usize) -> Vec<Note>;
    /// Applies serialist techniques (inversion, retrograde) to a row.
    fn apply_serial_techniques(&self, row: &[Note], technique: &str) -> Vec<Note>;
    /// Calculates the Shannon entropy of a melody.
    fn calculate_musical_entropy(&self, voice: &Voice) -> f64;
    
    // Performance and expression
    /// Adds articulation markings (e.g., staccato, legato) to a voice.
    fn add_articulation(&self, voice: &Voice, articulation: &str) -> Voice;
    /// Adds dynamic variations (e.g., crescendo, diminuendo) to a voice.
    fn add_dynamics(&self, voice: &Voice, dynamic_profile: &[(f64, u8)]) -> Voice;
    /// Adds tempo variations (rubato) to a voice.
    fn add_tempo_variations(&self, voice: &Voice, tempo_profile: &[(f64, f64)]) -> Voice;
    /// Generates musical ornamentation in a specific style.
    fn generate_ornamentation(&self, voice: &Voice, style: &str) -> Voice;
}

/// A concrete implementation of the `Bach` trait.
pub struct BachComposer {
    /// The musical temperament system to use (e.g., "equal").
    pub temperament: String,
    /// The tuning system to use (e.g., "12-tone").
    pub tuning_system: String,
}

impl Default for BachComposer {
    fn default() -> Self {
        Self {
            temperament: "equal".to_string(),
            tuning_system: "12-tone".to_string(),
        }
    }
}

impl Bach for BachComposer {
    fn note_to_frequency(&self, note: Note, octave: i32) -> f64 {
        let base_freq = match note {
            Note::C => 261.63,
            Note::CSharp => 277.18,
            Note::D => 293.66,
            Note::DSharp => 311.13,
            Note::E => 329.63,
            Note::F => 349.23,
            Note::FSharp => 369.99,
            Note::G => 392.00,
            Note::GSharp => 415.30,
            Note::A => 440.00,
            Note::ASharp => 466.16,
            Note::B => 493.88,
        };
        base_freq * 2.0_f64.powi(octave - 4)
    }
    
    fn frequency_to_note(&self, frequency: f64) -> (Note, i32) {
        let a4_freq = 440.0;
        let semitones = (frequency / a4_freq).log2() * 12.0;
        let octave = (semitones / 12.0).floor() as i32 + 4;
        let note_index = (semitones % 12.0).round() as i32;
        
        let note = match note_index {
            0 => Note::A,
            1 => Note::ASharp,
            2 => Note::B,
            3 => Note::C,
            4 => Note::CSharp,
            5 => Note::D,
            6 => Note::DSharp,
            7 => Note::E,
            8 => Note::F,
            9 => Note::FSharp,
            10 => Note::G,
            11 => Note::GSharp,
            _ => Note::A,
        };
        
        (note, octave)
    }
    
    fn interval_between(&self, note1: Note, note2: Note) -> Interval {
        let note_values = [
            Note::C, Note::CSharp, Note::D, Note::DSharp, Note::E, Note::F,
            Note::FSharp, Note::G, Note::GSharp, Note::A, Note::ASharp, Note::B
        ];
        
        let index1 = note_values.iter().position(|&n| n == note1).unwrap_or(0);
        let index2 = note_values.iter().position(|&n| n == note2).unwrap_or(0);
        let semitones = (index2 as i32 - index1 as i32 + 12) % 12;
        
        match semitones {
            0 => Interval::Unison,
            1 => Interval::MinorSecond,
            2 => Interval::MajorSecond,
            3 => Interval::MinorThird,
            4 => Interval::MajorThird,
            5 => Interval::PerfectFourth,
            6 => Interval::Tritone,
            7 => Interval::PerfectFifth,
            8 => Interval::MinorSixth,
            9 => Interval::MajorSixth,
            10 => Interval::MinorSeventh,
            11 => Interval::MajorSeventh,
            _ => Interval::Unison,
        }
    }
    
    fn transpose_note(&self, note: Note, interval: Interval) -> Note {
        let note_values = [
            Note::C, Note::CSharp, Note::D, Note::DSharp, Note::E, Note::F,
            Note::FSharp, Note::G, Note::GSharp, Note::A, Note::ASharp, Note::B
        ];
        
        let current_index = note_values.iter().position(|&n| n == note).unwrap_or(0);
        let new_index = (current_index + interval as usize) % 12;
        note_values[new_index]
    }
    
    fn generate_scale(&self, root: Note, scale: Scale) -> Vec<Note> {
        let intervals = match scale {
            Scale::Major => vec![0, 2, 4, 5, 7, 9, 11],
            Scale::NaturalMinor => vec![0, 2, 3, 5, 7, 8, 10],
            Scale::HarmonicMinor => vec![0, 2, 3, 5, 7, 8, 11],
            Scale::MelodicMinor => vec![0, 2, 3, 5, 7, 9, 11],
            Scale::Dorian => vec![0, 2, 3, 5, 7, 9, 10],
            Scale::Phrygian => vec![0, 1, 3, 5, 7, 8, 10],
            Scale::Lydian => vec![0, 2, 4, 6, 7, 9, 11],
            Scale::Mixolydian => vec![0, 2, 4, 5, 7, 9, 10],
            Scale::Locrian => vec![0, 1, 3, 5, 6, 8, 10],
            Scale::Chromatic => vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            Scale::WholeTone => vec![0, 2, 4, 6, 8, 10],
            Scale::Pentatonic => vec![0, 2, 4, 7, 9],
        };
        
        intervals.iter().map(|&interval| {
            self.transpose_note(root, match interval {
                0 => Interval::Unison,
                1 => Interval::MinorSecond,
                2 => Interval::MajorSecond,
                3 => Interval::MinorThird,
                4 => Interval::MajorThird,
                5 => Interval::PerfectFourth,
                6 => Interval::Tritone,
                7 => Interval::PerfectFifth,
                8 => Interval::MinorSixth,
                9 => Interval::MajorSixth,
                10 => Interval::MinorSeventh,
                11 => Interval::MajorSeventh,
                _ => Interval::Unison,
            })
        }).collect()
    }
    
    fn scale_degree(&self, note: Note, root: Note, scale: Scale) -> Option<u8> {
        let scale_notes = self.generate_scale(root, scale);
        scale_notes.iter().position(|&n| n == note).map(|pos| pos as u8 + 1)
    }
    
    fn is_in_scale(&self, note: Note, root: Note, scale: Scale) -> bool {
        let scale_notes = self.generate_scale(root, scale);
        scale_notes.contains(&note)
    }
    
    fn build_chord(&self, root: Note, chord_type: ChordType) -> Vec<Note> {
        let intervals = match chord_type {
            ChordType::Major => vec![0, 4, 7],
            ChordType::Minor => vec![0, 3, 7],
            ChordType::Diminished => vec![0, 3, 6],
            ChordType::Augmented => vec![0, 4, 8],
            ChordType::MajorSeventh => vec![0, 4, 7, 11],
            ChordType::MinorSeventh => vec![0, 3, 7, 10],
            ChordType::DominantSeventh => vec![0, 4, 7, 10],
            ChordType::DiminishedSeventh => vec![0, 3, 6, 9],
            ChordType::HalfDiminished => vec![0, 3, 6, 10],
            ChordType::SuspendedSecond => vec![0, 2, 7],
            ChordType::SuspendedFourth => vec![0, 5, 7],
        };
        
        intervals.iter().map(|&interval| {
            self.transpose_note(root, match interval {
                0 => Interval::Unison,
                2 => Interval::MajorSecond,
                3 => Interval::MinorThird,
                4 => Interval::MajorThird,
                5 => Interval::PerfectFourth,
                6 => Interval::Tritone,
                7 => Interval::PerfectFifth,
                8 => Interval::MinorSixth,
                9 => Interval::MajorSixth,
                10 => Interval::MinorSeventh,
                11 => Interval::MajorSeventh,
                _ => Interval::Unison,
            })
        }).collect()
    }
    
    fn chord_symbol(&self, chord: &Chord) -> String {
        let root_symbol = match chord.root {
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B",
        };
        
        let type_symbol = match chord.chord_type {
            ChordType::Major => "",
            ChordType::Minor => "m",
            ChordType::Diminished => "dim",
            ChordType::Augmented => "aug",
            ChordType::MajorSeventh => "maj7",
            ChordType::MinorSeventh => "m7",
            ChordType::DominantSeventh => "7",
            ChordType::DiminishedSeventh => "dim7",
            ChordType::HalfDiminished => "m7b5",
            ChordType::SuspendedSecond => "sus2",
            ChordType::SuspendedFourth => "sus4",
        };
        
        format!("{}{}", root_symbol, type_symbol)
    }
    
    fn analyze_chord(&self, notes: &[Note]) -> Option<Chord> {
        if notes.is_empty() {
            return None;
        }
        
        let root = notes[0];
        let intervals: Vec<Interval> = notes.iter().skip(1).map(|&note| {
            self.interval_between(root, note)
        }).collect();
        
        // Simple chord type detection
        let chord_type = if intervals.contains(&Interval::MajorThird) && intervals.contains(&Interval::PerfectFifth) {
            ChordType::Major
        } else if intervals.contains(&Interval::MinorThird) && intervals.contains(&Interval::PerfectFifth) {
            ChordType::Minor
        } else if intervals.contains(&Interval::MinorThird) && intervals.contains(&Interval::Tritone) {
            ChordType::Diminished
        } else {
            ChordType::Major // Default
        };
        
        Some(Chord {
            root,
            chord_type,
            octave: 4,
            duration: 1.0,
        })
    }
    
    fn chord_inversions(&self, chord: &Chord) -> Vec<Vec<Note>> {
        let notes = self.build_chord(chord.root, chord.chord_type);
        let mut inversions = Vec::new();
        
        for i in 0..notes.len() {
            let mut inversion = Vec::new();
            for j in 0..notes.len() {
                let note_index = (i + j) % notes.len();
                inversion.push(notes[note_index]);
            }
            inversions.push(inversion);
        }
        
        inversions
    }
    
    fn generate_progression(&self, key: Note, scale: Scale, length: usize) -> Progression {
        let scale_notes = self.generate_scale(key, scale);
        let mut chords = Vec::new();
        
        // Common chord progressions
        let progressions = match scale {
            Scale::Major => vec![0, 5, 3, 4], // I-V-vi-IV
            Scale::NaturalMinor => vec![0, 5, 3, 4], // i-V-III-IV
            _ => vec![0, 4, 1, 5], // Generic
        };
        
        for i in 0..length {
            let root_index = progressions[i % progressions.len()];
            let root = scale_notes[root_index];
            let chord_type = if root_index == 0 || root_index == 3 || root_index == 4 {
                ChordType::Major
            } else {
                ChordType::Minor
            };
            
            chords.push(Chord {
                root,
                chord_type,
                octave: 4,
                duration: 1.0,
            });
        }
        
        Progression {
            chords,
            key,
            scale,
        }
    }
    
    fn analyze_progression(&self, progression: &Progression) -> Vec<String> {
        progression.chords.iter().map(|chord| {
            self.chord_symbol(chord)
        }).collect()
    }
    
    fn voice_leading(&self, chord1: &Chord, chord2: &Chord) -> Vec<Voice> {
        let notes1 = self.build_chord(chord1.root, chord1.chord_type);
        let notes2 = self.build_chord(chord2.root, chord2.chord_type);
        
        let mut voices = Vec::new();
        for (i, &note) in notes1.iter().enumerate() {
            let target_note = if i < notes2.len() { notes2[i] } else { note };
            voices.push(Voice {
                notes: vec![(note, chord1.duration), (target_note, chord2.duration)],
                octave: chord1.octave,
                velocity: 80,
            });
        }
        
        voices
    }
    
    fn generate_counterpoint(&self, cantus_firmus: &Voice, rules: &CounterpointRules) -> Voice {
        let mut counterpoint = Voice {
            notes: Vec::new(),
            octave: cantus_firmus.octave + 1,
            velocity: cantus_firmus.velocity,
        };
        
        for (note, duration) in &cantus_firmus.notes {
            // Simple counterpoint: parallel thirds
            let counter_note = self.transpose_note(*note, Interval::MajorThird);
            counterpoint.notes.push((counter_note, *duration));
        }
        
        counterpoint
    }
    
    fn check_counterpoint_rules(&self, voice1: &Voice, voice2: &Voice, rules: &CounterpointRules) -> Vec<String> {
        let mut violations = Vec::new();
        
        for (i, ((note1, _), (note2, _))) in voice1.notes.iter().zip(voice2.notes.iter()).enumerate() {
            let interval = self.interval_between(*note1, *note2);
            
            if !rules.parallel_fifths_allowed && interval == Interval::PerfectFifth {
                violations.push(format!("Parallel fifths at position {}", i));
            }
            
            if !rules.parallel_octaves_allowed && interval == Interval::Unison {
                violations.push(format!("Parallel octaves at position {}", i));
            }
        }
        
        violations
    }
    
    fn invert_melody(&self, voice: &Voice, interval: Interval) -> Voice {
        Voice {
            notes: voice.notes.iter().map(|(note, duration)| {
                (self.transpose_note(*note, interval), *duration)
            }).collect(),
            octave: voice.octave,
            velocity: voice.velocity,
        }
    }
    
    fn retrograde_melody(&self, voice: &Voice) -> Voice {
        let mut retrograde_notes = voice.notes.clone();
        retrograde_notes.reverse();
        
        Voice {
            notes: retrograde_notes,
            octave: voice.octave,
            velocity: voice.velocity,
        }
    }
    
    fn generate_fugue_subject(&self, key: Note, scale: Scale) -> Voice {
        let scale_notes = self.generate_scale(key, scale);
        let mut subject = Voice {
            notes: Vec::new(),
            octave: 4,
            velocity: 80,
        };
        
        // Generate a simple fugue subject
        let subject_notes = vec![
            (scale_notes[0], 1.0), // Root
            (scale_notes[2], 0.5), // Third
            (scale_notes[4], 0.5), // Fifth
            (scale_notes[2], 1.0), // Third
            (scale_notes[0], 1.0), // Root
        ];
        
        subject.notes = subject_notes;
        subject
    }
    
    fn generate_fugue_answer(&self, subject: &Voice, key: Note) -> Voice {
        // Transpose the subject up a perfect fifth
        self.invert_melody(subject, Interval::PerfectFifth)
    }
    
    fn generate_fugue_exposition(&self, subject: &Voice, key: Note, voices: usize) -> Vec<Voice> {
        let mut exposition = Vec::new();
        
        for i in 0..voices {
            if i == 0 {
                exposition.push(subject.clone());
            } else if i == 1 {
                exposition.push(self.generate_fugue_answer(subject, key));
            } else {
                // Additional voices enter with the subject
                let transposed = self.invert_melody(subject, Interval::Octave);
                exposition.push(transposed);
            }
        }
        
        exposition
    }
    
    fn generate_canon(&self, subject: &Voice, interval: Interval, delay: f64) -> Vec<Voice> {
        let voice1 = subject.clone();
        let voice2 = self.invert_melody(subject, interval);
        
        // Add delay to voice2
        let mut delayed_voice2 = Voice {
            notes: vec![(Note::C, delay)], // Rest for delay
            octave: voice2.octave,
            velocity: voice2.velocity,
        };
        delayed_voice2.notes.extend(voice2.notes);
        
        vec![voice1, delayed_voice2]
    }
    
    fn calculate_harmonic_series(&self, fundamental: f64, partials: usize) -> Vec<f64> {
        (1..=partials).map(|n| fundamental * n as f64).collect()
    }
    
    fn calculate_just_intonation(&self, note: Note, octave: i32) -> f64 {
        let base_freq = match note {
            Note::C => 261.63,
            Note::CSharp => 261.63 * 25.0 / 24.0,
            Note::D => 261.63 * 9.0 / 8.0,
            Note::DSharp => 261.63 * 6.0 / 5.0,
            Note::E => 261.63 * 5.0 / 4.0,
            Note::F => 261.63 * 4.0 / 3.0,
            Note::FSharp => 261.63 * 45.0 / 32.0,
            Note::G => 261.63 * 3.0 / 2.0,
            Note::GSharp => 261.63 * 8.0 / 5.0,
            Note::A => 261.63 * 5.0 / 3.0,
            Note::ASharp => 261.63 * 9.0 / 5.0,
            Note::B => 261.63 * 15.0 / 8.0,
        };
        base_freq * 2.0_f64.powi(octave - 4)
    }
    
    fn calculate_equal_temperament(&self, note: Note, octave: i32) -> f64 {
        self.note_to_frequency(note, octave)
    }
    
    fn calculate_beat_frequency(&self, freq1: f64, freq2: f64) -> f64 {
        (freq1 - freq2).abs()
    }
    
    fn generate_rhythm_pattern(&self, meter: (u8, u8), complexity: f64) -> Vec<f64> {
        let (beats, beat_value) = meter;
        let mut pattern = Vec::new();
        
        for i in 0..beats as usize {
            let duration = if i == 0 { 1.0 } else { 0.5 };
            pattern.push(duration);
        }
        
        pattern
    }
    
    fn syncopate_rhythm(&self, rhythm: &[f64], syncopation_level: f64) -> Vec<f64> {
        rhythm.iter().map(|&duration| {
            if syncopation_level > 0.5 {
                duration * 0.75
            } else {
                duration
            }
        }).collect()
    }
    
    fn polyrhythm(&self, rhythm1: &[f64], rhythm2: &[f64]) -> Vec<(f64, f64)> {
        let lcm = rhythm1.len().lcm(rhythm2.len());
        let mut result = Vec::new();
        
        for i in 0..lcm {
            let r1 = rhythm1[i % rhythm1.len()];
            let r2 = rhythm2[i % rhythm2.len()];
            result.push((r1, r2));
        }
        
        result
    }
    
    fn analyze_melody(&self, voice: &Voice) -> HashMap<String, f64> {
        let mut analysis = HashMap::new();
        
        let note_count = voice.notes.len() as f64;
        let total_duration: f64 = voice.notes.iter().map(|(_, duration)| duration).sum();
        
        analysis.insert("note_count".to_string(), note_count);
        analysis.insert("total_duration".to_string(), total_duration);
        analysis.insert("average_duration".to_string(), total_duration / note_count);
        
        analysis
    }
    
    fn find_motifs(&self, voice: &Voice, min_length: usize) -> Vec<Vec<(Note, f64)>> {
        let mut motifs = Vec::new();
        
        for start in 0..voice.notes.len() {
            for length in min_length..=voice.notes.len() - start {
                if start + length <= voice.notes.len() {
                    motifs.push(voice.notes[start..start + length].to_vec());
                }
            }
        }
        
        motifs
    }
    
    fn calculate_tension(&self, progression: &Progression) -> Vec<f64> {
        progression.chords.iter().map(|chord| {
            match chord.chord_type {
                ChordType::Major => 0.0,
                ChordType::Minor => 0.3,
                ChordType::Diminished => 0.7,
                ChordType::Augmented => 0.8,
                ChordType::DominantSeventh => 0.6,
                _ => 0.2,
            }
        }).collect()
    }
    
    fn calculate_cadence_strength(&self, progression: &Progression) -> f64 {
        if progression.chords.len() < 2 {
            return 0.0;
        }
        
        let last_chord = &progression.chords[progression.chords.len() - 1];
        let second_last = &progression.chords[progression.chords.len() - 2];
        
        match (second_last.chord_type, last_chord.chord_type) {
            (ChordType::DominantSeventh, ChordType::Major) => 1.0,
            (ChordType::DominantSeventh, ChordType::Minor) => 0.8,
            (ChordType::Major, ChordType::Major) => 0.9,
            _ => 0.3,
        }
    }
    
    fn generate_bach_style_chorale(&self, hymn_tune: &Voice) -> Vec<Voice> {
        let mut chorale = Vec::new();
        
        // Generate four-part harmony
        for i in 0..4 {
            let octave = match i {
                0 => hymn_tune.octave + 1, // Soprano
                1 => hymn_tune.octave,     // Alto
                2 => hymn_tune.octave - 1, // Tenor
                3 => hymn_tune.octave - 2, // Bass
                _ => hymn_tune.octave,
            };
            
            let voice = Voice {
                notes: hymn_tune.notes.clone(),
                octave,
                velocity: 70,
            };
            
            chorale.push(voice);
        }
        
        chorale
    }
    
    fn generate_bach_style_prelude(&self, key: Note, scale: Scale) -> Voice {
        let scale_notes = self.generate_scale(key, scale);
        let mut prelude = Voice {
            notes: Vec::new(),
            octave: 4,
            velocity: 75,
        };
        
        // Generate arpeggiated patterns
        for _ in 0..16 {
            for &note in &scale_notes[..4] {
                prelude.notes.push((note, 0.25));
            }
        }
        
        prelude
    }
    
    fn generate_bach_style_fugue(&self, subject: &Voice, key: Note) -> Vec<Voice> {
        self.generate_fugue_exposition(subject, key, 3)
    }
    
    fn apply_bach_ornamentation(&self, voice: &Voice, ornamentation_level: f64) -> Voice {
        let mut ornamented = voice.clone();
        
        if ornamentation_level > 0.5 {
            // Add trills and mordents
            for i in 0..ornamented.notes.len() {
                if i % 4 == 0 {
                    let (note, duration) = ornamented.notes[i];
                    let trill_note = self.transpose_note(note, Interval::MajorSecond);
                    ornamented.notes.insert(i + 1, (trill_note, duration * 0.25));
                    ornamented.notes.insert(i + 2, (note, duration * 0.25));
                }
            }
        }
        
        ornamented
    }
    
    fn apply_transformation_matrix(&self, voice: &Voice, matrix: &[[f64; 4]; 4]) -> Voice {
        // Apply affine transformation to note positions
        let mut transformed = voice.clone();
        
        for (note, duration) in &mut transformed.notes {
            // Simple transformation: transpose based on matrix
            let transposition = (matrix[0][0] * *duration) as i32 % 12;
            *note = self.transpose_note(*note, match transposition {
                0 => Interval::Unison,
                1 => Interval::MinorSecond,
                2 => Interval::MajorSecond,
                3 => Interval::MinorThird,
                4 => Interval::MajorThird,
                5 => Interval::PerfectFourth,
                6 => Interval::Tritone,
                7 => Interval::PerfectFifth,
                8 => Interval::MinorSixth,
                9 => Interval::MajorSixth,
                10 => Interval::MinorSeventh,
                11 => Interval::MajorSeventh,
                _ => Interval::Unison,
            });
        }
        
        transformed
    }
    
    fn generate_serial_row(&self, length: usize) -> Vec<Note> {
        let all_notes = [
            Note::C, Note::CSharp, Note::D, Note::DSharp, Note::E, Note::F,
            Note::FSharp, Note::G, Note::GSharp, Note::A, Note::ASharp, Note::B
        ];
        
        let mut row = Vec::new();
        let mut used = vec![false; 12];
        let mut rng = rand::thread_rng();
        for _ in 0..length {
            let available: Vec<usize> = (0..12).filter(|&i| !used[i]).collect();
            if let Some(&index) = available.choose(&mut rng) {
                row.push(all_notes[index]);
                used[index] = true;
            }
        }
        row
    }
    
    fn apply_serial_techniques(&self, row: &[Note], technique: &str) -> Vec<Note> {
        match technique {
            "retrograde" => row.iter().rev().cloned().collect(),
            "inversion" => {
                let first = row[0];
                row.iter().map(|&note| {
                    let interval = self.interval_between(first, note);
                    self.transpose_note(first, interval)
                }).collect()
            },
            "retrograde_inversion" => {
                let inverted = self.apply_serial_techniques(row, "inversion");
                inverted.iter().rev().cloned().collect()
            },
            _ => row.to_vec(),
        }
    }
    
    fn calculate_musical_entropy(&self, voice: &Voice) -> f64 {
        let note_count = voice.notes.len() as f64;
        if note_count == 0.0 {
            return 0.0;
        }
        
        let mut note_frequencies = HashMap::new();
        for (note, _) in &voice.notes {
            *note_frequencies.entry(note).or_insert(0) += 1;
        }
        
        let mut entropy = 0.0;
        for &count in note_frequencies.values() {
            let probability = count as f64 / note_count;
            if probability > 0.0 {
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }
    
    fn add_articulation(&self, voice: &Voice, articulation: &str) -> Voice {
        let mut articulated = voice.clone();
        
        match articulation {
            "staccato" => {
                for (_, duration) in &mut articulated.notes {
                    *duration *= 0.5;
                }
            },
            "legato" => {
                for (_, duration) in &mut articulated.notes {
                    *duration *= 1.2;
                }
            },
            _ => {},
        }
        
        articulated
    }
    
    fn add_dynamics(&self, voice: &Voice, dynamic_profile: &[(f64, u8)]) -> Voice {
        let mut dynamic_voice = voice.clone();
        
        for (i, _) in dynamic_voice.notes.iter().enumerate() {
            if let Some(&(_, vel)) = dynamic_profile.get(i % dynamic_profile.len()) {
                dynamic_voice.velocity = vel;
            }
        }
        
        dynamic_voice
    }
    
    fn add_tempo_variations(&self, voice: &Voice, tempo_profile: &[(f64, f64)]) -> Voice {
        let mut tempo_voice = voice.clone();
        
        for (i, (_, duration)) in tempo_voice.notes.iter_mut().enumerate() {
            if let Some(&(_, tempo_factor)) = tempo_profile.get(i % tempo_profile.len()) {
                *duration *= tempo_factor;
            }
        }
        
        tempo_voice
    }
    
    fn generate_ornamentation(&self, voice: &Voice, style: &str) -> Voice {
        match style {
            "baroque" => self.apply_bach_ornamentation(voice, 0.8),
            "classical" => self.add_articulation(voice, "staccato"),
            "romantic" => self.add_articulation(voice, "legato"),
            _ => voice.clone(),
        }
    }
}

// Helper function for LCM calculation
trait Lcm {
    fn lcm(&self, other: usize) -> usize;
}

impl Lcm for usize {
    fn lcm(&self, other: usize) -> usize {
        self * other / self.gcd(other)
    }
}

trait Gcd {
    fn gcd(&self, other: usize) -> usize;
}

impl Gcd for usize {
    fn gcd(&self, other: usize) -> usize {
        let mut a = *self;
        let mut b = other;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}