const program = require('commander');

program
    .version('0.1.0')
    .option('-n, --namespace <string>', 'namespace for the secrets')
    .parse(process.argv);

console.log(`namespace ${program.namespace}`);
program.args.forEach(secret => {
    console.log(secret);
});
