#![recursion_limit = "1024"]
use std::vec::Vec;
use std::ops::Range;

#[macro_use] 
extern crate lazy_static;

extern crate regex;

extern crate rust_htslib;
use rust_htslib::bam::record::Cigar;

extern crate bio;

extern crate csv;

#[macro_use]
extern crate failure;

extern crate structopt;

#[macro_use]
extern crate duct;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

extern crate linked_hash_map;

#[macro_use]
extern crate url;

extern crate itertools;

extern crate unindent;

pub mod indexed_annotation;

pub mod error {
    pub type Result<T> = ::std::result::Result<T, ::failure::Error>;

    #[derive(Debug, Fail)]
    enum NoneError {
        #[fail(display = "Option value is None")]
        NoneError {}
    }

    pub trait ToResult<T> {
        fn r(self) -> Result<T>;
    }
    impl<T> ToResult<T> for Option<T> {
        fn r(self) -> Result<T> {
            match self {
                Some(v) => Ok(v),
                None => Err(NoneError::NoneError{}.into()),
            }
        }
    }
}
use ::error::*;

pub mod power_set {
    pub struct PowerSet<'a, T: 'a> {
        source: &'a [T],
        position: usize
    }
    
    impl<'a, T> PowerSet<'a, T> where T: Clone {
        pub fn new(source: &'a [T]) -> PowerSet<'a, T> {
            PowerSet { source: source, position: 0 }
        }
    }

    impl<'a, T> Iterator for PowerSet<'a, T> where T: Clone {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            if 2usize.pow(self.source.len() as u32) <= self.position {
                None
            } else {
                let res = self.source.iter().enumerate().filter(|&(i, _)| (self.position >> i) % 2 == 1)
                                                        .map(|(_, element)| element.clone()).collect();
                self.position = self.position + 1;
                Some(res)
            }
        }
    }
}

pub fn cigar2exons(cigar: &[Cigar], pos: u64) -> Result<Vec<Range<u64>>> {
    let mut exons = Vec::<Range<u64>>::new();
    let mut pos = pos;
    for op in cigar {
        match op {
            &Cigar::Match(length) => {
                pos += length as u64;
                if length > 0 {
                    exons.push(Range{start: pos - length as u64, end: pos});
                }
            }
            &Cigar::RefSkip(length) |
            &Cigar::Del(length) |
            &Cigar::Equal(length) |
            &Cigar::Diff(length) => {
                pos += length as u64;
            }
            &Cigar::Back(length) => {
                pos -= length as u64;
            }
            &Cigar::Ins(_) |
            &Cigar::SoftClip(_) |
            &Cigar::HardClip(_) |
            &Cigar::Pad(_) => (),
        };
    }
    Ok(exons)
}
