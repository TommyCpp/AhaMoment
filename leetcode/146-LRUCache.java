class LRUCache {
    HashMap<Integer,Integer> hash = new HashMap();
    Deque<Integer> queue;
    int size;

    public LRUCache(int capacity) {
        hash = new HashMap();
        queue = new ArrayDeque(capacity);
        size = capacity;
    }
    
    public int get(int key) {
        if(hash.get(key) == null){
            return -1;
        }
        else{
            queue.removeFirstOccurrence(key);
            queue.addLast(key);
        }
        return hash.get(key);
    }
    
    public void put(int key, int value) {
        if(hash.get(key) != null){
            hash.put(key,value);
            queue.removeFirstOccurrence(key);
            queue.addLast(key);
            return;
        }
        if(queue.size() == this.size){
            int removeKey = queue.poll();
            hash.remove(removeKey);
        }
        hash.put(key, value);
        queue.addLast(key);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache obj = new LRUCache(capacity);
 * int param_1 = obj.get(key);
 * obj.put(key,value);
 */
