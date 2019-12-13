
trait Descendable<T: Descendable> {
    fn descend(key: String) -> T;
}
