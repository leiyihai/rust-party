# Rust Party — 对话记录

## 会话: 创建 rust-code-reviewer Skill (2026-07-15)

**用户：** 提供了 `rust-code-reviewer/SKILL.md` 的开头（Role + Review Philosophy + 不完整的 Review Process），要求在 `rust-code-reviewer` 里补充完善这个 skill。

**AI 执行：**

1. 调研项目结构 — 发现 `rust-code-reviewer/SKILL.md`（51行，825字节）和 `rust-mentor/SKILL.md`（250行，3575字节）
2. 参考 `rust-mentor` 的风格和结构，将 `rust-code-reviewer` 从 51 行扩充到 313 行 / 6902 字节
3. 新增内容：
   - Correctness Check（含 4 级严重级别标签）
   - Ownership & Borrowing 分析（核心）
   - API Design 评估
   - Error Handling 检查
   - Performance 审查
   - Testing 检查
   - Style & Idioms
   - Response Format 模板
   - Teaching During Review（引导式提问 + 学习资源推荐）
   - Priority Rules（5 条行为准则）
   - Reviewing Unsafe Code（含审查标注格式）
   - Audit Mode（深度审查清单）
   - Golden Rule

**用户后续提问：** "根据已有的这2个skill，足够我开始学习rust了吗？"

**AI 回答：** 足够。rust-mentor 内置了 Stage 1-5 的完整学习路径，rust-code-reviewer 提供代码审查闭环。用户可以随时开始学习。

**用户后续提问：** "帮我把项目推送到 GitHub，对话记录有办法也推送保存吗？"

**AI 执行（当前任务）：**
1. 创建 `.gitignore`
2. 导出本对话记录到 `.conversation/`
3. 初始化 git 仓库并提交
4. 推送到 `https://github.com/leiyihai/rust-party.git`
