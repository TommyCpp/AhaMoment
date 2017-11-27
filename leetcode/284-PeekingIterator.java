// Java Iterator interface reference:
// https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html
class PeekingIterator implements Iterator<Integer> {
    Iterator<Integer> iterator;
    Integer a;
    Integer b;

	public PeekingIterator(Iterator<Integer> iterator) {
	    // initialize any member here.
	    this.iterator = iterator;
        if(iterator.hasNext()){
            this.a = iterator.next();
            if(iterator.hasNext()){
                this.b = iterator.next();
            }
        }
	}

    // Returns the next element in the iteration without advancing the iterator.
	public Integer peek() {
        return this.a;
	}

	// hasNext() and next() should behave the same as in the Iterator interface.
	// Override them if needed.
	@Override
	public Integer next() {
        Integer result = this.a;
        this.a = this.b;
        if(this.iterator.hasNext()){
            this.b = this.iterator.next();
        }
        else{
            this.b = null;
        }
        return result;
	}

	@Override
	public boolean hasNext() {
        if(this.a == null)
            return false;
        return true;
	}
}
