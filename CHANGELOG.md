# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Fixed
-Adopt libm for sqrt operations.  Micromatch was not accurate enough for practical use in droop controls.

### Changes
-Change all transforms to to use 0 degree phase alignment (cos centric)
-Added `rotate` and `rotated` methods for `Dq` data type to distribuish between in place rotation and returning a new rotated vector


### Added
-Added more algabraic operations to the reference frames
-Add line phasor to sequence phasor transforms
-Break transforms into multiple files
-Added a `UnitVector` which is a struct that contains a cos, sin pair from a common angle


## v0.1.0

### Fixed
### Changes
### Added
- The initial release