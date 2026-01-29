# VibeTerm 상세 스펙 문서 (PRD)

## 1. 프로젝트 개요

- **명칭:** VibeTerm (The Terminal for Vibe Coding)
- **개발 환경:** macOS (Apple Silicon 및 Intel)
- **현재 버전:** v0.7.1 (Preferences Window)

### 핵심 철학 (Core Philosophy)

**VibeTerm**은 단순히 명령어를 입력하는 창을 넘어, **'바이브 코딩(Vibe Coding)'**에 최적화된 차세대 터미널 에뮬레이터입니다. Claude Code, Codex와 같은 AI CLI 도구를 사용하는 개발자가 터미널 환경을 떠나지 않고도 IDE 이상의 생산성을 경험하도록 설계되었습니다.

* **The Orchestrator:** 터미널은 단순한 텍스트 출력을 넘어, AI와 개발자가 공유하는 지능형 캔버스가 되어야 합니다.
* **Environment Control:** AI 도구가 실행되는 부모 환경(Terminal Emulator)을 지능화하여 컨텍스트를 주입하고 결과를 시각화합니다.

---

## 2. 기술 스택 (Tech Stack)

### 현재 스택

| 구성요소 | 라이브러리 | 버전 | 역할 |
|---------|-----------|------|------|
| 언어 | Rust | Stable | 메모리 안전 시스템 프로그래밍 |
| GUI Framework | egui + eframe | 0.31 | Immediate Mode GUI |
| 터미널 위젯 | egui_term | 0.1 | Alacritty 백엔드 기반 터미널 |
| 네이티브 메뉴 | muda | 0.15 | macOS 네이티브 메뉴바 |
| 설정 | serde + toml | 1.0 / 0.8 | 설정 파일 직렬화 |
| 비동기 | tokio | 1.0 | PTY 비동기 처리 |
| 로깅 | log + env_logger | 0.4 / 0.11 | 디버깅 로그 |

### v0.7.0에 추가됨

| 구성요소 | 라이브러리 | 버전 | 역할 |
|---------|-----------|------|------|
| 파일 감시 | notify | 6.1 | 파일 시스템 변경 실시간 감지 |
| Git 통합 | git2 | 0.19 | Git 상태 추적 |
| 패턴 매칭 | regex | 1.10 | PTY 스트림 파싱 |

### 추가 예정 (v0.8.0+)

| 구성요소 | 라이브러리 | 역할 |
|---------|-----------|------|
| MCP 통신 | (TBD) | Model Context Protocol 지원 |
| LSP 통합 | (TBD) | Semantic Search 지원 |

### 이전 시도 (폐기됨)
- ~~Iced 기반~~: 한글 입력/렌더링 성능 이슈
- ~~ratatui TUI 기반~~: iTerm2 등 호스트 터미널과 키보드 충돌
- ~~wgpu 직접 렌더링~~: 복잡도 높음, egui_term으로 대체

---

## 3. 현재 아키텍처

### 레이아웃 구조 (v0.5.0 - Binary Split Tree)
```
+------------------------------------------+
| [macOS Native Menu Bar]                   |   <- System Menu
+------------------------------------------+
|  [Tab Bar]    ▶1 shell │ 2 file.rs   +   |   <- 24px (TUI style)
+--------+---------------------------------+
| [Side  |   [Terminal Area]               |
|  bar]  |   +-------------+-------------+ |
|        |   |  Pane 1    ║  Pane 2     | |
| Files  |   |  (focused) ║             | |
|        |   +-----+------+-------------+ |
|  Tree  |         ║  Pane 3            | |
|        |         ║  (nested split)    | |
+--------+---------------------------------+
|  [Status Bar]  VibeTerm │ Panes: 3 │ ... |   <- 18px
+------------------------------------------+

Drag & Drop: 패널을 드래그하여 Top/Bottom/Left/Right 영역에 드롭
```

### 상태 관리 구조 (v0.5.0)
```
VibeTermApp
├── config: Config                    // 사용자 설정
│   ├── theme: ThemeConfig           // 색상 테마
│   ├── font: FontConfig             // 폰트 설정
│   └── ui: UiConfig                 // UI 레이아웃
├── theme: RuntimeTheme              // 파싱된 Color32 값
├── cached_terminal_theme            // 캐시된 터미널 테마 (성능 최적화)
├── workspaces: Vec<Workspace>       // 탭별 워크스페이스
│   └── Workspace
│       ├── name: String             // 탭 이름
│       ├── root: LayoutNode<TabContent>  // Binary Split Tree (NEW)
│       ├── focused_pane: PaneId     // 포커스된 패널 ID
│       └── next_pane_id: u64        // 다음 패널 ID
├── active_workspace: usize          // 활성 탭
├── sidebar_entries: Vec<FileEntry>  // 파일 트리
├── sidebar_selected: Option<usize>  // 선택된 파일
├── show_preferences: bool           // 설정 창 표시
├── ime_composing: bool              // IME 조합 중 여부
├── dragging_divider: Option<...>    // 디바이더 드래그 상태
└── dragging_pane: Option<PaneDragState>  // 패널 드래그 상태 (NEW)
```

### Binary Split Tree 구조 (src/layout.rs)
```rust
pub enum LayoutNode<T> {
    Leaf { id: PaneId, content: T },
    Split {
        direction: SplitDirection,  // Horizontal | Vertical
        ratio: f32,                 // 0.0-1.0
        first: Box<LayoutNode<T>>,
        second: Box<LayoutNode<T>>,
    },
}
```

---

## 4. 구현 상태

### ✅ v0.7.1 완료 (Preferences Window & Documentation)

#### Preferences Window Implementation
- [x] 5개 탭 (General, Appearance, Terminal, File Tree, Advanced)
- [x] iTerm2 스타일 UI (좌측 탭 바, 우측 콘텐츠)
- [x] 모든 색상 설정 (UI + ANSI 16색)
- [x] 폰트 크기 슬라이더
- [x] Cancel/Apply/Save 버튼
- [x] 실시간 색상 미리보기
- [x] 헥스 색상 입력 검증
- [x] 모달 오버레이 (Esc/Cmd+W로 닫기)
- [x] `~/.config/vibeterm/config.toml` 저장

#### Documentation Updates (v0.7.1)
- [x] **PREFERENCES_GUIDE.md** (1000줄+)
  - 모든 5개 탭 설명 및 예제
  - 설정 파일 참조
  - 일반적인 작업 (테마 변경, 폰트 조정, 색상 커스터마이징)
  - 다양한 시스템을 위한 성능 팁
  - 문제 해결 섹션
- [x] **PERFORMANCE_OPTIMIZATION.md** (800줄+)
  - 메모리 프로파일링 및 예상 사용량
  - 7가지 최적화 전략
  - GPU 가속 상태 및 로드맵
  - 경쟁 제품과의 비교 (iTerm2, Ghostty, Terminal.app)
  - 프로파일링 도구 및 모니터링
  - 성능 문제 해결
  - 벤치마크 및 Phase 3 대상
- [x] **README.md** 문서 링크 추가
- [x] **SHORTCUTS.md** Preferences 섹션 추가
- [x] **CHANGELOG.md** v0.7.1 항목 추가

---

### ✅ v0.7.1 완료 (Preferences Window)

#### Preferences Window (iTerm2 스타일)
- [x] **완전한 Preferences 인터페이스** 5개 탭 섹션
  - General: 폰트 크기, 레이아웃 치수, 시작 동작
  - Appearance: 헥스 입력이 있는 UI 색상 커스터마이징
  - Terminal: ANSI 16색 팔레트 편집
  - File Tree: 사이드바 구성 및 무시 패턴
  - Advanced: 컨텍스트 엔진 및 성능 설정
- [x] **대화형 UI 요소**:
  - 헥스 입력 필드가 있는 색상 미리보기
  - 숫자 값용 슬라이더 (시각적 피드백)
  - 부울 설정용 체크박스
  - 패턴 구성용 텍스트 영역
- [x] **빠른 작업 버튼**:
  - Cancel: 변경사항 폐기
  - Apply: 즉시 적용 (저장 없음)
  - Save: 적용 및 `~/.config/vibeterm/config.toml`에 저장
- [x] **키보드 단축키**:
  - `Cmd+,`: Preferences 열기 (표준 macOS)
  - `Esc`: Preferences 닫기
  - `Cmd+W`: Preferences 닫기
- [x] **모달 오버레이** 반투명 배경
- [x] **반응형 레이아웃** 스크롤 가능한 콘텐츠 영역
- [x] **색상 변경의 실시간 미리보기**

---

### ✅ v0.6.0 완료

#### Multi-Pane Contextual Sidebar
- [x] **동적 사이드바 루트 전환**
  - 패널 포커스 변경 시 사이드바 자동 업데이트
  - 프로젝트 루트 자동 탐지 (.git, Cargo.toml, package.json)
  - 비동기 디렉토리 로딩 (최대 1000개 파일, 10 레벨)
- [x] **패널 인디케이터**
  - 사이드바 헤더에 열린 패널 목록
  - 클릭으로 패널 포커스 전환

#### 터미널 인터랙션
- [x] **스크롤백 버퍼**
  - 마우스 휠/트랙패드로 히스토리 스크롤
  - 자동 스크롤 (하단에 있을 때)
- [x] **텍스트 선택 및 복사**
  - 클릭-드래그로 텍스트 선택
  - 더블 클릭으로 단어 선택
  - 트리플 클릭으로 라인 선택
  - Cmd+C로 클립보드 복사

#### Command Palette
- [x] **Cmd+P 명령 팔레트**
  - 퍼지 검색 (9개 내장 명령)
  - 탭/패널 네비게이션
  - 키보드 단축키 표시

#### 탭 관리
- [x] **탭 드래그 앤 드롭**
  - 마우스로 탭 순서 재정렬
  - 5px 드래그 임계값
  - 고스트 프리뷰

---

### ✅ v0.7.0 완료 (Smart Context Management)

#### Context Management System (v0.7.0)
- [x] **파일 감시 서비스**
  - notify 6.1 크레이트로 파일 시스템 실시간 감지
  - 200ms 디바운싱으로 성능 최적화
  - 빌드 아티팩트 무시 (.git, target, node_modules)
- [x] **Git 상태 통합**
  - git2 0.19 크레이트로 저장소 상태 추적
  - 9가지 파일 상태 (Modified, Staged, Untracked, Deleted 등)
  - 5초 자동 캐시 새로고침
  - 브랜치/ahead/behind 정보 표시
- [x] **수동 파일 고정**
  - AI 컨텍스트용 파일 핀 기능 (📌)
  - LRU 제거 (최대 50개 파일)
  - 3가지 고정 이유: Manual, RecentlyEdited, TerminalMentioned
- [x] **이벤트 기반 아키텍처**
  - ContextManager로 모든 서브시스템 조정
  - ContextEvent 열거형으로 UI 업데이트

#### Smart Sidebar 업그레이드
- [x] **Git 상태 표시**
  - 파일명 앞 상태 인디케이터 (M, A, U, D, R, C, !)
  - 색상 코딩 (Modified=노랑, Staged=초록, Untracked=회색 등)
- [x] **핀 인디케이터**
  - 고정된 파일에 📌 이모지 표시
- [x] **전체 접기/펼치기**
  - Cmd+Shift+C: 모든 디렉토리 접기
  - Cmd+Shift+E: 모든 디렉토리 펼치기
  - UI 버튼 (⊟, ⊞) 추가

#### 성능 및 품질
- [x] **메모리 최적화**: <50MB 오버헤드
- [x] **CPU 효율**: <5% 유휴 사용률
- [x] **19개 단위 테스트** (100% 통과)

#### 연기된 기능 (v0.8.0으로)
- [ ] **PTY Interception** - 실시간 에러 감지 (egui_term 제약)
- [ ] **Semantic Search** - @-keyword 심볼 검색 (선택적 기능)

---

### ✅ 핵심 기반 기능 (v0.5.0 - 유지)

#### 터미널 렌더링
- [x] **egui_term 기반 터미널 렌더링**
  - Alacritty 백엔드 사용
  - PTY 프로세스 생성 및 통신
  - ANSI 이스케이프 시퀀스 완벽 지원
- [x] **Binary Split Tree 레이아웃**
  - 수평 분할 (Cmd+D) - left | right
  - 수직 분할 (Cmd+Shift+D) - top / bottom
  - 무한 중첩 분할 지원
  - 클릭으로 패널 포커스 전환
  - 디바이더 드래그로 크기 조절
  - Ctrl+Tab/Ctrl+Shift+Tab 패널 순환
- [x] **드래그 앤 드롭 패널 재배치**
  - iTerm2 스타일 패널 재배치
  - 8px 임계값 후 드래그 활성화
  - Top/Bottom/Left/Right 드롭 존 하이라이트
  - ESC로 드래그 취소
  - 트리 구조 extract/insert 함수
- [x] **성능 최적화**
  - O(n²) → O(n) 렌더 루프 최적화
  - 조건부 레이아웃 재계산
  - 터미널 테마 캐싱
  - 배치 입력 상태 읽기
  - 20fps 유휴 repaint (CPU 절약)

#### 탭 및 UI
- [x] **탭 시스템**
  - 새 탭 생성 (Cmd+T, + 버튼)
  - 탭 전환 (클릭, Cmd+1-9)
  - 탭 닫기 (Cmd+W, 중간 클릭)
- [x] **macOS 네이티브 메뉴바** (muda 크레이트)
- [x] **TUI 스타일 UI** (Box-drawing 문자)
- [x] **사이드바 파일 탐색기**
- [x] **설정 창** (Cmd+,)
- [x] **TOML 기반 테마 커스터마이징**

#### 폰트 및 국제화
- [x] **CJK 폰트 지원** (한글/일본어/중국어)
- [x] **IME 지원** (ViewportCommand::IMEAllowed)

---

## 5. Vibe Coding 기능 로드맵

### Phase 1: 기반 기능 완성 (v0.6.0) - **HIGH PRIORITY**

#### 🔥 Multi-Pane Contextual Sidebar (Dynamic File Tree) - **NEXT**
터미널 패널 전환 시 사이드바 파일 트리가 자동으로 해당 작업 디렉토리를 표시

| 요구사항 | 상세 |
|---------|------|
| **Core Behavior** | 패널 포커스 변경 → 사이드바 루트 자동 전환 |
| **Sidebar Header** | 열린 터미널 패널 목록 (아이콘/탭 형태) |
| **Smart Root Sync** | cd 명령 시 프로젝트 루트(.git, Cargo.toml) 자동 탐지 |
| **PTY Sync** | CWD 변경 감지 (portable-pty 이벤트 감시) |
| **Performance** | 비동기 디렉토리 로드 (UI 프리징 방지) |

**구현 가이드:**
```rust
// Pane 상태 확장
struct Pane {
    current_path: PathBuf,  // 현재 작업 디렉토리
    // ...
}

// 메시지 추가
enum Message {
    PaneFocused(PaneId),
    DirectoryChanged(PaneId, PathBuf),
    // ...
}
```

**User Story:**
1. 유저가 1번 패널(Backend)을 클릭 → 사이드바가 백엔드 프로젝트 폴더 표시
2. 유저가 2번 패널(Frontend)로 전환 → 사이드바가 프론트엔드 프로젝트 폴더 표시
3. 맥락 전환(Context Switching) 비용 최소화

---

#### 터미널 기본 기능
- [ ] 스크롤백 버퍼
- [ ] 텍스트 선택 및 복사
- [ ] 탭 드래그 앤 드롭 재정렬
- [ ] 새 창 열기 (Cmd+Shift+N)
- [ ] Command Palette (Cmd+P)

#### UI 개선
- [ ] 배경 투명도 (Opacity) 설정
- [ ] 배경 흐림 효과 (Blur)
- [ ] Active Line Highlight
- [ ] 커서 스타일 설정 (Block, Bar, Underline)

---

### Phase 2: 지능형 컨텍스트 관리 (v0.7.0) - ✅ **완료**

#### 🧠 Smart Context Management
- [x] **Auto-Context Pinning**
  - 수동 파일 고정 (📌) 기능
  - AI 세션 컨텍스트 관리
  - LRU 제거로 최대 50개 파일 유지
- [ ] **PTY Interception** → v0.8.0으로 연기
  - PTY 스트림 실시간 파싱
  - 에러 로그 및 특정 패턴 감지
  - 감지된 이벤트를 UI에 반영
- [ ] **Semantic Search** → 선택적 기능 (Phase 9)
  - `@` 키워드로 프로젝트 내 심볼(함수, 클래스) 검색
  - 프롬프트에 즉시 포함

#### Smart Sidebar 업그레이드
- [x] **Pinned Files 표시**
  - 고정된 파일에 📌 표시
- [x] **Git Status 통합**
  - 변경된 파일 시각적 표시 (M, A, U, D 등)
  - 색상 코딩
- [x] **File Watcher (`notify` 크레이트)**
  - 파일 시스템 변경 실시간 감지
  - 200ms 디바운싱
  - UI 자동 업데이트
- [x] **전체 접기/펼치기 기능**
  - 키보드 단축키와 UI 버튼

---

### Phase 3: 실시간 코드 적용 (v0.8.0)

#### ⚡ Actionable Output
- [ ] **Ghost Text & Diff Preview**
  - AI 제안 변경사항을 반투명 오버레이로 시각화
  - 별도 패널에서 상세 Diff 표시
- [ ] **One-tap Apply**
  - `y` 키로 제안 코드 즉시 적용
  - git commit 연동 옵션

#### AI Inspector 패널 (Right Panel)
- [ ] **Thought Trace**
  - AI의 추론 과정 실시간 표시
- [ ] **상세 Diff View**
- [ ] **토큰 사용량 모니터**
- [ ] **실시간 비용 리포트**

---

### Phase 4: 멀티 세션 오케스트레이션 (v0.9.0)

#### 🤝 Multi-Session / MCP
- [ ] **Claude Event Bus (MCP)**
  - Model Context Protocol 활용
  - 여러 터미널 세션 간 상태/이벤트 공유
- [ ] **Global Context Dashboard**
  - 여러 패널의 AI 작업 상태 통합 모니터링
- [ ] **Smart Handoff**
  - 세션 A의 맥락을 세션 B로 즉시 전송
  - 다른 모델/작업 영역 간 컨텍스트 이전

---

### Phase 5: Vibe UI 완성 (v1.0.0)

#### 🎨 The "Vibe" UI
- [ ] **Aura Effect**
  - AI 작업 중 터미널 테두리에 그라데이션 애니메이션
- [ ] **Smooth Animations**
  - 패널 전환, 탭 전환 애니메이션
- [ ] **Customizable Themes**
  - AI 전용 색상 (Aura, Highlight, Selection)

---

## 6. 우선순위 정렬 (Priority Matrix)

| 우선순위 | 기능 | 이유 | 예상 복잡도 |
|---------|------|------|------------|
| **P0** | Multi-Pane Contextual Sidebar | 멀티 프로젝트 워크플로우 핵심 | Medium |
| **P1** | 스크롤백 버퍼 | 터미널 기본 기능 | Low |
| **P1** | 텍스트 선택/복사 | 터미널 기본 기능 | Medium |
| **P2** | Command Palette | 파워 유저 생산성 | Medium |
| **P2** | 탭 드래그 재정렬 | UX 개선 | Low |
| **P3** | 배경 투명도/블러 | 미관 | Low |
| **P3** | Git Status 통합 | 컨텍스트 향상 | Medium |

---

## 7. 설정 명세 (Preferences Specification)

### 현재 구현된 설정

```toml
[theme]
background = "#2E1A16"
surface = "#3A241E"
primary = "#E07A5F"
text = "#F4F1DE"
text_dim = "#A0968A"
border = "#4A2E28"

[font]
family = "JetBrains Mono"
size = 13.0
```

### 목표 설정 스펙

#### ⚙️ 일반 및 외관 (General & Appearance)

```toml
[general]
default_shell = "/bin/zsh"
scrollback_lines = 10000
initial_directory = "~"

[font]
family = "JetBrains Mono"
size = 13.0
line_height = 1.2
ligatures = true

[window]
opacity = 1.0          # 0.0 - 1.0
blur = false
padding = 8

[cursor]
style = "block"        # block, bar, underline
blink_speed = 500      # ms, 0 to disable
```

#### 🎨 색상 테마 (Color Schemes)

```toml
[theme]
# Base colors
background = "#2E1A16"
surface = "#3A241E"
primary = "#E07A5F"
text = "#F4F1DE"
text_dim = "#A0968A"
border = "#4A2E28"

# ANSI 16 colors
black = "#2E1A16"
red = "#E07A5F"
green = "#81B29A"
yellow = "#F2CC8F"
blue = "#3D405B"
magenta = "#B5838D"
cyan = "#6D9DC5"
white = "#F4F1DE"

# Vibe special (AI 전용)
aura_color = "#E07A5F"
ai_text_highlight = "#81B29A"
selection_color = "#3A241E"
ghost_text_color = "#A0968A"
```

#### 🤖 AI/연동 설정 (AI Specifics)

```toml
[ai]
enabled = true
mcp_bus_path = "/tmp/vibeterm-mcp.sock"
mcp_servers = ["claude", "codex"]

[ai.cost]
token_budget_limit = 100000     # daily limit
warning_threshold = 0.8         # 80% 경고

[ai.context]
ignored_patterns = ["node_modules", ".git", "target"]
watcher_debounce_ms = 100
auto_pin_opened_files = true
```

---

## 8. 키보드 단축키

### 현재 구현됨

| 단축키 | 기능 |
|--------|------|
| `Cmd+T` | 새 탭 |
| `Cmd+W` | 현재 패널/탭 닫기 |
| `Cmd+D` | 수평 분할 |
| `Cmd+Shift+D` | 수직 분할 ✅ NEW |
| `Cmd+B` | 사이드바 토글 |
| `Cmd+,` | 설정 창 |
| `Cmd+1-9` | 탭 전환 |
| `Ctrl+Tab` | 다음 패널로 이동 |
| `Ctrl+Shift+Tab` | 이전 패널로 이동 |
| `ESC` | 드래그 취소 |

### 추가 예정

| 단축키 | 기능 |
|--------|------|
| `Cmd+Shift+N` | 새 창 |
| `Cmd+P` | Command Palette |
| `Cmd+Shift+P` | AI Command Palette |
| `@` | Semantic Search (터미널 내) |
| `y` | AI 제안 적용 |

---

## 9. 파일 구조

### 현재 구조 (v0.5.0)
```
src/
├── main.rs          # 진입점, eframe 실행
├── app.rs           # VibeTermApp (메인 애플리케이션 상태)
├── layout.rs        # Binary Split Tree 레이아웃 시스템 (NEW)
├── config.rs        # Config, ThemeConfig, RuntimeTheme
├── menu.rs          # 네이티브 메뉴바 (muda)
├── theme.rs         # 테마 적용, TUI 문자, CJK 폰트 로드
└── ui/
    ├── mod.rs       # UI 모듈 익스포트
    ├── tab_bar.rs   # 탭바 컴포넌트
    ├── sidebar.rs   # 사이드바 파일 탐색기
    └── status_bar.rs # 상태바 컴포넌트

~/.config/vibeterm/
└── config.toml      # 사용자 설정 파일
```

### 추가 예정 구조

```
src/
├── ai/
│   ├── mod.rs           # AI 모듈
│   ├── context.rs       # 컨텍스트 관리
│   ├── mcp.rs           # MCP 통신
│   └── inspector.rs     # AI Inspector 패널
├── watcher/
│   └── mod.rs           # 파일 감시 (notify)
└── ui/
    ├── command_palette.rs  # Cmd+P
    ├── ghost_text.rs       # AI 제안 오버레이
    └── ai_inspector.rs     # 우측 AI 패널
```

---

## 10. 빌드 및 실행

```bash
# 빌드
cargo build --release

# 실행
cargo run

# 로그 확인
RUST_LOG=info cargo run 2>&1 | tee vibeterm.log
```

---

## 11. 알려진 제한사항

### 한글 IME 이슈
- **현상**: 일부 환경에서 한글 입력 시 자음/모음이 분리
- **원인**: winit/egui의 네이티브 IME 지원 한계
  - [winit#1497](https://github.com/rust-windowing/winit/issues/1497)
  - [egui#248](https://github.com/emilk/egui/issues/248)

---

## 12. 핸드오프 노트

### 마지막 작업 상태 (2025-01-27)

#### v0.5.0-v0.7.0 축적된 완성도
- ✅ egui_term 마이그레이션 완료
- ✅ 네이티브 메뉴바 구현
- ✅ 패널 포커스 전환 (클릭)
- ✅ 디바이더 드래그 (크기 조절)
- ✅ 파일 더블클릭으로 새 탭 열기
- ✅ TOML 기반 테마 커스터마이징
- ✅ CJK 폰트 자동 로드
- ✅ 한글 IME 지원 (ViewportCommand::IMEAllowed)
- ✅ **Binary Split Tree 레이아웃 구현** (v0.5.0)
- ✅ **수직 분할 Cmd+Shift+D** (v0.5.0)
- ✅ **드래그 앤 드롭 패널 재배치** (v0.5.0)
- ✅ **O(n²) → O(n) 렌더 루프 최적화** (v0.5.0)
- ✅ **동적 사이드바 루트 전환** (v0.6.0)
- ✅ **스크롤백 버퍼** (v0.6.0)
- ✅ **텍스트 선택 및 복사** (v0.6.0)
- ✅ **Command Palette (Cmd+P)** (v0.6.0)
- ✅ **탭 드래그 앤 드롭** (v0.6.0)
- ✅ **파일 감시 서비스** (v0.7.0)
- ✅ **Git 상태 통합** (v0.7.0)
- ✅ **파일 고정 (핀) 기능** (v0.7.0)
- ✅ **이벤트 기반 아키텍처** (v0.7.0)
- ✅ **19개 단위 테스트** (v0.7.0)

### 다음 우선순위 작업 (v0.8.0+)
1. **Ghost Text & Diff Preview** - AI 제안 코드 시각화
2. **One-tap Apply** - `y` 키로 제안 즉시 적용
3. **AI Inspector Panel** - 우측 패널에서 실시간 AI 작업 모니터링
4. **PTY Interception** - 실시간 에러 감지 및 패턴 매칭
5. **MCP 통신** - Model Context Protocol 지원
6. **LSP 통합** - Semantic Search (`@`-keyword 심볼 검색, 선택적)
