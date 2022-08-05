const listSelectors = require("list-selectors");
const fs = require("fs");

listSelectors(
    ["./demo/styles/output.css"],
    { include: ["classes"] },
    ({ classes }) => {
        fs.writeFileSync(
            "./demo/styles/usedCSSClasses.json",
            JSON.stringify(
                classes.map(c =>
                    c
                        .substring(1)
                        .split("\\")
                        .join("")
                )
            )
        );
    }
)