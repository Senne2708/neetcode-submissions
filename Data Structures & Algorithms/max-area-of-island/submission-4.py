class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        self.ROW, self.COL = len(grid), len(grid[0])
        self.maxArea = 0
        self.direction = [[1, 0], [0, 1], [-1, 0], [0, -1]]

        def dfs(r, c):
            if r < 0 or c < 0 or r >= self.ROW or c >= self.COL or grid[r][c] == 0:
                return 0
            grid[r][c] = 0
            area = 1
            for dr, dc in self.direction:
                area += dfs(r + dr, c + dc)
            return area

        for row in range(self.ROW):
            for col in range(self.COL):
                if grid[row][col] == 1:
                    self.maxArea = max(self.maxArea, dfs(row, col))

        return self.maxArea
        