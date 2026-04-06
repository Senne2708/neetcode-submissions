class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        row, col = len(grid), len(grid[0])
        island = 0
        directions = [[1, 0], [0, 1], [-1, 0], [0, -1]]

        def dsf(r, c):
            if (r < 0) or (c < 0) or (r >= row) or (c >= col) or (grid[r][c] == "0") :
                return
            
            grid[r][c] = "0"
            for dr, dc in directions:
                dsf(r+dr, c+dc)


        for r in range(row):
            for c in range(col):
                if grid[r][c] == "1":
                    dsf(r, c)
                    island+=1
        
        return island