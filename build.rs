fn main() {
    cc::Build::new().file("c_src/timestring.c").compile("libtimestring.a");
}
