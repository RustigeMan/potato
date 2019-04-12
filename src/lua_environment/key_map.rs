use sfml::window::Key;
use std::collections::HashMap;

pub fn gen_key_map() -> HashMap<String, Key> {
    let mut keys = HashMap::new();

    let mut addk = |s: &str, k: Key| {
        keys.insert(s.to_string(), k);
    };

    use Key::*;
    addk("Escape", Escape);

    addk("F1", F1);
    addk("F2", F2);
    addk("F3", F3);
    addk("F4", F4);
    addk("F5", F3);
    addk("F6", F3);
    addk("F7", F3);
    addk("F8", F3);
    addk("F9", F9);
    addk("F10", F10);
    addk("F11", F11);
    addk("F12", F12);

    addk("Up", Up);
    addk("Down", Down);
    addk("Left", Left);
    addk("Right", Right);

    addk("LControl", LControl);
    addk("RControl", RControl);
    addk("LAlt", LAlt);
    addk("RAlt", RAlt);
    addk("LShift", LShift);
    addk("RShift", RShift);
    addk("Space", Space);
    addk("Return", Return);
    addk("Delete", Delete);
    addk("BackSpace", BackSpace);

    addk("0", Num0);
    addk("1", Num1);
    addk("2", Num2);
    addk("3", Num3);
    addk("4", Num4);
    addk("5", Num5);
    addk("6", Num6);
    addk("7", Num7);
    addk("8", Num8);
    addk("9", Num9);

    addk("A", A);
    addk("B", B);
    addk("C", C);
    addk("D", D);
    addk("E", E);
    addk("F", F);
    addk("G", G);
    addk("H", H);
    addk("I", I);
    addk("J", J);
    addk("K", K);
    addk("L", L);
    addk("M", M);
    addk("N", N);
    addk("O", O);
    addk("P", P);
    addk("Q", Q);
    addk("R", R);
    addk("S", S);
    addk("T", T);
    addk("U", U);
    addk("V", V);
    addk("W", W);
    addk("X", X);
    addk("Y", Y);
    addk("Z", Z);

    addk("=", Equal);
    addk("[", LBracket);
    addk("]", RBracket);
    addk("\\", BackSlash);
    addk(";", SemiColon);
    addk("'", Quote);
    addk(",", Comma);
    addk(".", Period);
    addk("/", Slash);

    keys
}
