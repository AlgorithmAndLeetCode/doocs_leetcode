class Solution {
    func permutation(_ S: String) -> [String] {
        var ans: [String] = []
        var s: [Character] = Array(S).sorted()
        var t: [Character] = Array(repeating: " ", count: s.count)
        var vis: [Bool] = Array(repeating: false, count: s.count)
        let n = s.count

        func dfs(_ i: Int) {
            if i >= n {
                ans.append(String(t))
                return
            }
            for j in 0..<n {
                if !vis[j] && (j == 0 || s[j] != s[j - 1] || vis[j - 1]) {
                    vis[j] = true
                    t[i] = s[j]
                    dfs(i + 1)
                    vis[j] = false
                }
            }
        }

        dfs(0)
        return ans
    }
}
