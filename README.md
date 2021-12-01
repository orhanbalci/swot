# Swot

[![Crates.io](https://img.shields.io/crates/v/swot.svg)](https://crates.io/crates/swot)
[![Documentation](https://docs.rs/swot/badge.svg)](https://docs.rs/swot)
[![License](https://img.shields.io/github/license/orhanbalci/swot.svg)](https://github.com/orhanbalci/swot/blob/master/LICENSE)

If you have a product or service and offer **academic discounts**, there's a good chance there's some manual component to the approval process. Perhaps `.edu` email addresses are automatically approved because, for the most part at least, they're associated with American post-secondary educational institutions. Perhaps `.ac.uk` email addresses are automatically approved because they're guaranteed to belong to British universities and colleges. Unfortunately, not every country has an education-specific TLD (Top Level Domain) and plenty of schools use `.com` or `.net`.

Swot is a community-driven or crowdsourced library for verifying that domain names and email addresses are tied to a legitimate university of college - more specifically, an academic institution providing higher education in tertiary, quaternary or any other kind of post-secondary education in any country in the world.

**Pop quiz:** Which of the following domain names should be eligible for an academic discount? `stanford.edu`, `america.edu`, `duep.edu`, `gla.ac.uk`, `unizar.es`, `usask.ca`, `hil.no`, `unze.ba`, `fu-berlin.de`, `ecla.de`, `bvb.de`, `lsmu.com`. Answers at the foot of the page.

### Installation

```toml
[dependencies]
swot="0.1.0"
```


### Usage

```rust
use swot::{is_academic, get_school_names}
```

#### Verify Email Addresses

```rust
is_academic("abadojack@students.uonbi.ac.ke")	//true
is_academic("pedro@ugr.es")				        //true
is_academic("abadojack@leerilly.net")		    //false
is_academic("abadojack@gmail.com")			    //false
```

#### Verify Domain Names

```rust
is_academic("harvard.edu")              //true
is_academic("www.harvard.edu")          //true
is_academic("http://www.harvard.edu")   //true
is_academic("http://www.github.com")    //false
is_academic("http://www.rangers.co.uk") //false
```

#### Find School Names

```rust
get_school_names("abadojack@students.uonbi.ac.ke")
# => "University of Nairobi"

get_school_names("http://www.stanford.edu")
# => "Stanford University"
```

### Known Issues

* You can search by email and domain names only. You cannot search by IP.
* You don't know if the email address belongs to a student, faculty, staff member, alumni, or a contractor.
* There may be a few false positives, missing institutions... maybe even a couple of typos. Contributions welcome!

**Please note:** just because someone has verified that they own `lreilly@stanford.edu` does *not* mean that they're a student. They could be faculty, staff, alumnni, or maybe even an external contractor. If you're suddenly getting a lot of traffic from websites like [FatWallet](http://www.fatwallet.com) or [SlickDeals](http://www.slickdeals.net), you might want to find out why. If you're suddenly getting a lot of requests from a particular school, you should look into that too. It may be good business, word of mouth, or someone may have found a loophole. Swot gives you a *high confidence level* - not a guarantee. I recommend putting some controls in place or at least monitor how it's doing from time to time.


### Pop Quiz Answers

Hopefully, you'll be surprised by some of this:

| Domain | Academic? | Comments |
|--------|-----------|----------|
|`stanford.edu`|:heavy_check_mark:|OK, this was an easy one so you could get at least *one* right|
|`america.edu`|:heavy_multiplication_x:| Prior to October 29th 2001, anyone could register a `.edu` domain name ([details](https://en.wikipedia.org/wiki/.edu#Grandfathered_uses)) |
|`duep.edu`|:heavy_check_mark:| Alfred Nobel University is a *Ukranian* University *in the Ukraine* i.e. not in the USA :us: |
|`gla.ac.uk`|:heavy_check_mark:|Glasgow University in Scotland|
|`unizar.es`|:heavy_check_mark:|The University of Zaragoza in Spain|
|`usask.ca`|:heavy_check_mark:|The University of Saskatchewan in Canada|
|`hil.no`|:heavy_check_mark:|Lillehammer University College in Norway|
|`unze.ba`|:heavy_check_mark:|University of Zenica in Bosnia and Herzegovina|
|`fu-berlin.de`|:heavy_check_mark:|Free University of Berlin in Germany|
|`ecla.de`|:heavy_check_mark:|ECLA of Bard is a state recognized liberal arts university in Berlin, Germany |
|`bvb.de`|:heavy_multiplication_x:|It's a soccer team from Germany|
|`lsmu.com`|:heavy_check_mark:| Lugansk State Medical University in the Ukraine |

If you verified this by visiting all of the websites, how long did it take you? Did you have fun? Imagine you had to do this 10 - 100 times every day. Now you know a little something about the inspiration for Swot. Swot can verify them all in a fraction of a second and remove a :poop: part of someone's job.

### See Also
* [swot](https://github.com/leereilly/swot) - Swot in Ruby (Original swot. Inspired the rest of of the swot ports including this.)
* [gman](https://github.com/benbalter/gman) - like swot, but for government emails
* [swotphp](https://github.com/mdwheele/swotphp) - PHP port of Swot
* [swot-js](https://github.com/theotow/swot-js) - JS port of Swot
* [swot-simple](https://github.com/mapbox/swot-simple) - JS port of Swot
* [swot-clj](https://github.com/ipavl/swot-clj) - Clojure port of Swot

