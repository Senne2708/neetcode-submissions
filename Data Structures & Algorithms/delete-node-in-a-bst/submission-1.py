# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        if not root:
            return root

        if root.val < key:
            root.right = self.deleteNode(root.right, key)
        elif root.val > key:
            root.left = self.deleteNode(root.left, key)
        else:
            if not root.right:
                return root.left
            elif not root.left:
                return root.right
            else:
                minVal = self.findMin(root.right)
                root.val = minVal.val
                root.right = self.deleteNode(root.right, minVal.val)
        return root


        

    def findMin(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        cur = root
        while cur and root.left:
            cur = root.left

        return cur
