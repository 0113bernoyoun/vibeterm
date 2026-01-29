# VibeTerm

**몰입을 끊지 마세요.**

![Version](https://img.shields.io/badge/version-0.7.1-blue)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey)
![Rust](https://img.shields.io/badge/rust-stable-orange)

[English](./README.md)

## 왜 VibeTerm인가?

코딩에 완전히 몰입한 순간. 아이디어가 물 흐르듯 나오는 그 타이밍. 앱을 전환하고, 창을 찾고, 컨텍스트를 바꾸느라 흐름이 끊기는 건 최악입니다.

**VibeTerm은 바이브 코더를 위해 만들어졌습니다** — Claude Code, Codex 같은 AI CLI 도구를 사용하며, 터미널을 벗어나지 않고도 IDE 이상의 생산성을 원하는 개발자를 위한 터미널입니다.

- **전환 말고, 분할하세요.** 여러 터미널을 나란히 열어두세요. ⌘+Tab 지옥은 이제 그만.
- **파일도 바로 여기서.** 터미널을 벗어나지 않고 파일을 탐색하고 열 수 있습니다.
- **나만의 워크스페이스.** 탭, 패널, 테마 — 내 생각의 흐름대로 구성하세요.
- **네이티브 & 빠름.** Rust로 제작. macOS의 일부처럼 자연스럽습니다.

```
+------------------------------------------+
| [macOS 네이티브 메뉴바]                    |
+------------------------------------------+
|  [탭 바]      ▶1 shell │ 2 file.rs   +   |
+--------+---------------------------------+
| [사이드 |   [터미널 영역]                  |
|   바]  |   +-------------+-------------+ |
|        |   |  패널 1    ║  패널 2      | |
| 파일   |   |  (포커스)  ║             | |
|        |   +============+-------------+ |
|  트리  |   디바이더 (드래그 가능)         |
+--------+---------------------------------+
|  [상태 바]  VibeTerm │ 패널: 2           |
+------------------------------------------+
```

## 기능

### 멀티 패널 워크스페이스 (P0)
- **수평/수직 분할** — 터미널을 나란히 정렬
- **자동 전환 사이드바** — 포커스된 패널의 디렉토리 추적
- **스마트 프로젝트 감지** — .git, Cargo.toml, package.json 자동 인식
- **패널 인디케이터** — 사이드바 헤더의 클릭 가능한 탭
- **비동기 로딩** — 최대 1000개 파일, 10 레벨 깊이 지원
- **드래그 크기 조절** — 부드러운 패널 디바이더

### 터미널 텍스트 인터랙션 (P1)
- **스크롤백 버퍼** — 터미널 히스토리 스크롤
- **텍스트 선택** — 클릭-드래그, 더블클릭 단어, 트리플클릭 라인
- **클립보드 복사** — `Cmd+C`로 선택된 텍스트 복사

### 명령 팔레트 & 탭 관리 (P2)
- **명령 팔레트** — `Cmd+P` / `Ctrl+P` 퍼지 검색 (9개 내장 명령)
- **탭 드래그 앤 드롭** — 마우스로 탭 순서 재정렬
- **빠른 네비게이션** — 탭과 패널 간 즉시 전환

### 모든 것을 한 곳에서
- 사이드바에 통합된 파일 탐색기
- 여러 컨텍스트를 위한 멀티 탭
- macOS 네이티브 메뉴바

### 속도를 위해 설계됨
- Alacritty 기반 터미널 백엔드
- ANSI 이스케이프 시퀀스 완벽 지원
- 비동기 PTY 통신

### 나만의 스타일로
- 다크 브라운 테마 (Preferences 창에서 완전 커스터마이징 가능)
- CJK 폰트 지원 (한글/일본어/중국어)
- IME 입력 지원
- 고급 Preferences 창 (iTerm2 스타일) 완전 커스터마이징

## 로드맵

**VibeTerm**은 바이브 코딩을 위한 궁극의 터미널로 진화하고 있습니다 — 개발자와 AI가 함께 공유하는 지능형 캔버스가 됩니다.

### 완료됨

| 버전 | 기능 |
|------|------|
| **v0.5** | 수직 분할, 스크롤백, 텍스트 선택, Command Palette |
| **v0.6** | 멀티 패널 컨텍스트 사이드바, 탭 재정렬, 명령 팔레트 ✓ |
| **v0.7** | 컨텍스트 관리 (Git 상태, 파일 핀, 파일 감시자) ✓ |

### 진행 중

| 단계 | 기능 |
|------|------|
| **v0.8** | Ghost Text 프리뷰, 원탭 적용, AI Inspector 패널 |
| **v0.9** | MCP 통합, 멀티 세션 오케스트레이션, 스마트 핸드오프 |
| **v1.0** | Aura 효과, 부드러운 애니메이션, 완전한 AI 통합 |

전체 로드맵은 [vibeterm_specification.md](./vibeterm_specification.md)를 참고하세요.

## 설치

### 요구사항
- macOS (Apple Silicon / Intel)
- Rust (Stable)

### 빌드

```bash
git clone https://github.com/0113bernoyoun/vibeterm.git
cd vibeterm
cargo build --release
cargo run --release
```

## 키보드 단축키

### 탭 & 패널 네비게이션
| 단축키 | 기능 |
|--------|------|
| `Cmd+T` | 새 탭 |
| `Cmd+W` | 현재 탭 닫기 |
| `Cmd+D` | 수평 분할 |
| `Cmd+Shift+D` | 수직 분할 |
| `Cmd+1-9` | 탭 전환 (1-9) |
| `Ctrl+Tab` | 다음 탭 |
| `Ctrl+Shift+Tab` | 이전 탭 |

### 사이드바 & UI
| 단축키 | 기능 |
|--------|------|
| `Cmd+B` | 사이드바 토글 |
| `Cmd+Shift+C` | 사이드바의 모든 디렉토리 접기 |
| `Cmd+Shift+E` | 사이드바의 모든 디렉토리 펼치기 |
| `Cmd+,` | Preferences 창 열기 |

### 명령 팔레트
| 단축키 | 기능 |
|--------|------|
| `Cmd+P` | 명령 팔레트 열기 (macOS) |
| `Ctrl+P` | 명령 팔레트 열기 (Linux) |

### 텍스트 선택 & 상호작용
| 단축키 | 기능 |
|--------|------|
| 클릭 + 드래그 | 텍스트 선택 |
| 더블클릭 | 단어 선택 |
| 트리플클릭 | 라인 선택 |
| `Cmd+C` | 선택된 텍스트 복사 |
| 스크롤 휠 | 스크롤백 버퍼 |

## 설정

### 빠른 설정

`Cmd+,`로 Preferences 창을 열어 다음을 커스터마이징하세요:
- **폰트 크기** (터미널 및 UI)
- **색상** (UI 및 ANSI 터미널 팔레트)
- **사이드바** (파일 트리 제한, 무시 패턴)
- **레이아웃** (사이드바 너비, 탭/상태바 높이)
- **고급** (컨텍스트 엔진, 성능 옵션)

자세한 설정 문서는 [PREFERENCES_GUIDE.md](/Users/bernocrest/Desktop/dev/projects/vibeterm/PREFERENCES_GUIDE.md)를 참고하세요.

### 수동 설정

설정 파일: `~/.config/vibeterm/config.toml`

```toml
[theme]
background = "#2E1A16"
surface = "#3A241E"
primary = "#E07A5F"
text = "#F4F1DE"
text_dim = "#A0968A"
border = "#4A2E28"

[font]
terminal_size = 14.0
ui_size = 12.0

[ui]
sidebar_width = 220.0
show_sidebar = true
max_files = 1000
max_depth = 10
```

## 설명서

- **[PREFERENCES_GUIDE.md](./PREFERENCES_GUIDE.md)** - Preferences 창 커스터마이징 완벽 가이드
- **[PERFORMANCE_OPTIMIZATION.md](./PERFORMANCE_OPTIMIZATION.md)** - 메모리 및 성능 튜닝
- **[SHORTCUTS.md](./SHORTCUTS.md)** - 모든 키보드 단축키 참조
- **[CHANGELOG.md](./CHANGELOG.md)** - 버전 히스토리 및 업데이트

## 기술 스택

| 구성요소 | 라이브러리 | 버전 |
|---------|-----------|------|
| 언어 | Rust | Stable |
| GUI | egui + eframe | 0.31 |
| 터미널 | egui_term | 0.1 |
| 메뉴 | muda | 0.15 |
| 설정 | serde + toml | 1.0 / 0.8 |
| 비동기 | tokio | 1.0 |

## 알려진 제한사항

- **한글 IME**: winit/egui의 IME 지원 한계로 인해 일부 환경에서 한글 입력이 불완전할 수 있습니다.

## 라이선스

MIT License

## 기여

Pull request를 환영합니다. 큰 변경사항의 경우, 먼저 이슈를 열어 변경하고자 하는 내용을 논의해주세요.
