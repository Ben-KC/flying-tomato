# Flying Tomato

## A blazing fast command-line Pomodoro timer, written in Rust

The [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique) is a helpful method of breaking tasks into 
intervals of work and break. A number of tools already exist for timing and tracking there intervals, but they all have 
one thing in common: they are incredibly slow. Flying Tomato seeks to solve that problem.

Being built in Rust, this Pomodoro timer is memory-safe, cross-platform, and blazing fast. Where most timers can take 
as long as 25 minutes to get through a work interval, Flying Tomato completes that same interval in under 25 
***seconds***!

## See it in action!

![Alt Text: Pomofocus.io is opened in a browser window and its pomodoro timer is started. A GNOME pomodoro is then opened and started.
Finally, a terminal is opened and ./flying_tomato is slowly typed in. When the return key is hit, a TUI pomodoro timer appears.
The time decreases rapidly, and the TUI pomodoro timer flies through a work and a break interval in the time it takes the other
timers to get just a couple of minutes into their respective work intervals.](media/flying-tomato.GIF)

## Installation

To install Flying Tomato, you can either build from source or download the appropriate executable from the [Releases 
page](https://github.com/Ben-KC/flying-tomato/releases).

## Usage

1. In your terminal, navigate to wherever you've placed the executable (or add it to PATH)
2. Execute the executable
3. Blaze through work intervals faster than you ever have before!

## FAQS

* That's not how this works, that's not how any of this works.  
**Correct**


* Why are you picking on Rust?  
**Because I really love Rust, but it's kinda silly that so many Rust projects feel obligated to describe themselves 
as "blazing fast."**  
 

* Is this going to become a real, usable Pomodoro timer?  
**Not at the moment. I was looking for a good way to start learning how to use the 
[`tui`](https://crates.io/crates/tui) crate and thought this would be a fun way to do that. I might still come back and 
build on this as a continued learning opportunity, though.**
