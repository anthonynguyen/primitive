error_chain!{
    foreign_links {
        Image(::image::ImageError);
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}
