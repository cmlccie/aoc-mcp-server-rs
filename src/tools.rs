use rmcp::{
    Error as McpError, RoleServer, ServerHandler, model::*, schemars, service::RequestContext, tool,
};

#[derive(Clone)]
pub struct Solver {}

#[tool(tool_box)]
impl Solver {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "Solve the puzzle for the specified day and part for the provided input.")]
    fn solve_puzzle(
        &self,
        #[tool(param)]
        #[schemars(description = "The puzzle day (1-25).")]
        day: u8,
        #[tool(param)]
        #[schemars(description = "The puzzle part (1 or 2).")]
        part: u8,
        #[tool(param)]
        #[schemars(description = "The puzzle input.")]
        input: String,
    ) -> Result<CallToolResult, McpError> {
        let answer = match (day, part) {
            (1, 1) => aoc24::day1::part1(&input),
            (1, 2) => aoc24::day1::part2(&input),
            (2, 1) => aoc24::day2::part1(&input),
            (2, 2) => aoc24::day2::part2(&input),
            (3, 1) => aoc24::day3::part1(&input),
            (3, 2) => aoc24::day3::part2(&input),
            (4, 1) => aoc24::day4::part1(&input),
            (4, 2) => aoc24::day4::part2(&input),
            (5, 1) => aoc24::day5::part1(&input),
            (5, 2) => aoc24::day5::part2(&input),
            (6, 1) => aoc24::day6::part1(&input),
            (6, 2) => aoc24::day6::part2(&input),
            (7, 1) => aoc24::day7::part1(&input),
            (7, 2) => aoc24::day7::part2(&input),
            (8, 1) => aoc24::day8::part1(&input),
            (8, 2) => aoc24::day8::part2(&input),
            (9, 1) => aoc24::day9::part1(&input),
            (9, 2) => aoc24::day9::part2(&input),
            (10, 1) => aoc24::day10::part1(&input),
            (10, 2) => aoc24::day10::part2(&input),
            (11, 1) => aoc24::day11::part1(&input),
            (11, 2) => aoc24::day11::part2(&input),
            (12, 1) => aoc24::day12::part1(&input),
            (12, 2) => aoc24::day12::part2(&input),
            (13, 1) => aoc24::day13::part1(&input),
            (13, 2) => aoc24::day13::part2(&input),
            (14, 1) => aoc24::day14::part1(&input, 101, 103),
            (14, 2) => aoc24::day14::part2(&input, false),
            (15, 1) => aoc24::day15::part1(&input),
            (15, 2) => aoc24::day15::part2(&input),
            (16, 1) => aoc24::day16::part1(&input),
            (16, 2) => aoc24::day16::part2(&input),
            (17, 1) => aoc24::day17::part1(&input),
            (17, 2) => aoc24::day17::part2(&input),
            (18, 1) => aoc24::day18::part1(&input, 70, 1024),
            (18, 2) => aoc24::day18::part2(&input, 70, 1024),
            (19, 1) => aoc24::day19::part1(&input),
            (19, 2) => aoc24::day19::part2(&input),
            (20, 1) => aoc24::day20::part1(&input),
            (20, 2) => aoc24::day20::part2(&input),
            (21, 1) => aoc24::day21::part1(&input),
            (21, 2) => aoc24::day21::part2(&input),
            (22, 1) => aoc24::day22::part1(&input),
            (22, 2) => aoc24::day22::part2(&input),
            (23, 1) => aoc24::day23::part1(&input),
            (23, 2) => aoc24::day23::part2(&input),
            (24, 1) => aoc24::day24::part1(&input),
            (24, 2) => aoc24::day24::part2(&input),
            (25, 1) => aoc24::day25::part1(&input),
            _ => {
                return Err(McpError::new(
                    ErrorCode::INVALID_PARAMS,
                    "Unsupported day or part.",
                    None,
                ));
            }
        };

        let response = if let Some(answer) = answer {
            format!(
                "Solved puzzle for day {} part {} with the provided input, answer is: {}",
                day, part, answer
            )
        } else {
            format!(
                "Failed to solve puzzle for day {} part {} with the provided input.",
                day, part
            )
        };

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }
}

#[tool(tool_box)]
impl ServerHandler for Solver {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server solves Advent of Code puzzles for the 2024 edition.".to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}
