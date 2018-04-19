class Solution {
    public int islandPerimeter(int[][] grid) {
        int result = 0;
        int countRow = grid.length;
        int countCol = grid[0].length;
        for(int i = 0; i < grid.length; i++){
            for(int j = 0; j < grid[i].length; j++){
                if(grid[i][j] == 1){
                    if(i == 0){
                        result += 1;
                    }
                    else{
                         if(grid[i-1][j] == 0){
                            result += 1;
                        }
                    }
                    if(i == countRow - 1){
                        result += 1;
                    }
                    else{
                        if(grid[i+1][j] == 0){
                            result += 1;
                        }
                    }
                    
                    if(j == 0){
                        result += 1;
                    }
                    else{
                        if(grid[i][j-1] == 0){
                            result += 1;
                        }
                    }
                    if(j == countCol - 1){
                        result += 1;
                    }
                    else{
                        if(grid[i][j+1] == 0){
                            result += 1;
                        }
                    }
                    
                }
            }
        }
        return result;
    }
}
