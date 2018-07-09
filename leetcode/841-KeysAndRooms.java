class Solution {
    public boolean canVisitAllRooms(List<List<Integer>> rooms) {
        List<Boolean> opened = new ArrayList(rooms.size());
        opened.add(true);
        for(int i = 1; i < rooms.size();i ++){
            opened.add(false);
        }
        //opened数组指明房间是否打开过
        List<Integer> keys = rooms.get(0);
        List<Integer> nextKeys = new ArrayList();
        while(!keys.isEmpty()){
            for(int key : keys){
                if(!opened.get(key)){
                    //仅仅打开没打开过的房间
                    opened.set(key, true);
                    nextKeys.addAll(rooms.get(key));
                }
            }
            keys = nextKeys;
            nextKeys = new ArrayList();
        }
        
        for(int i = 0; i < rooms.size(); i++){
            if(!opened.get(i)){
                return false;
            }
        }
        return true;
    }
}
