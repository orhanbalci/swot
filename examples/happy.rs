fn main() {
    dbg!(swot::is_academic("abadojack@students.uonbi.ac.ke")); //true
    dbg!(swot::is_academic("pedro@ugr.es")); //true
    dbg!(swot::is_academic("abadojack@leerilly.net")); //false
    dbg!(swot::is_academic("abadojack@gmail.com")); //false

    dbg!(swot::is_academic("harvard.edu")); //true
    dbg!(swot::is_academic("www.harvard.edu")); //true
    dbg!(swot::is_academic("http://www.harvard.edu")); //true
    dbg!(swot::is_academic("http://www.github.com")); //false
    dbg!(swot::is_academic("http://www.rangers.co.uk")); //false

    dbg!(swot::get_school_names("abadojack@students.uonbi.ac.ke"));
    dbg!(swot::get_school_names("http://www.stanford.edu"));
}
