if !variable::get("use_default_licensing") {
    print("NOTE: remember to add license information in the initial commit!");
    file::delete("LICENSE.md");
    file::delete("LICENSE-Apache-2.0.md");
    file::delete("LICENSE-MIT.md");
}
