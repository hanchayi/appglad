# Welcome to the webpack test suite!!!!
欢迎来到webpack测试套件!!!!

Every pull request that you submit to webpack (besides README and spelling corrections in comments) requires tests that are created.
你提交给webpack的每一个pull request(除了README和注释中的拼写更正之外)都需要创建测试。

But don't give up hope!!! Although our tests may appear complex and overwhelming, once you become familiar with the test suite and structure, adding and creating tests will be fun and beneficial as you work inside the codebase! ❤
但是不要放弃希望!!虽然我们的测试可能看起来很复杂，让人不知所措，但一旦你熟悉了测试套件和结构，添加和创建测试将是有趣的，因为你在代码库中工作!❤


Run all tests (this automatically runs the setup):
运行所有测试(这将自动运行安装程序):

``` bash
yarn test
```

Run an individual suite:
运行单个套件:

``` bash
yarn jest ConfigTestCases
```

Watch mode:
监听模式

``` bash
yarn jest --watch ConfigTestCases
```

See also: Jest CLI docs
另见:Jest CLI文档



## Test suite overview
测试套件概述

We use Jest for our tests. For more information on Jest you can visit their homepage!
我们使用Jest进行测试。有关Jest的更多信息，您可以访问他们的主页!

### Class Tests
类测试

All test files can be found in *.test.js. There are many tests that simply test APIs of a specific class/file (such as Compiler, Errors, Integration, Parser, RuleSet, Validation). If the feature you are contributing involves one of those classes, then best to start there to understand the structure.
所有测试文件都可以在*.test.js中找到。
有许多测试只是测试特定类/文件的api(例如编译器、错误、集成、解析器、规则集、验证)。
如果您正在贡献的功能涉及其中一个类，那么最好从那里开始了解结构。


### xCases
In addition to Class specific tests, there are also directories that end in "Cases". The suites for these cases also have corresponding *.test.js files.
除了特定于类的测试之外，还有以“case”结尾的目录。
这些情况下的套件也有相应的*.test.js文件。

### cases (TestCases.test.js) 1
Cases are a set of general purpose tests that will run against a variety of permutations of webpack configurations. When you are making a general purpose change that doesn't require you to have a special configuration, you would likely add your tests here. Inside of the ./test/cases directory you will find tests are broken into thematic sub directories. Take a moment to explore the different options.
用例是一组通用的测试，它们将针对各种排列的webpack配置运行。
当你做一个不需要特殊配置的通用更改时，你可能会在这里添加测试。
在./test/cases目录下，你会发现测试被分解为主题子目录。
花点时间来探索不同的选择。

To add a new case, create a new directory inside of the top level test groups, and then add an index.js file (and any other supporting files).
要添加新用例，需要在顶级测试组中创建一个新目录，然后添加index.js文件(以及其他支持文件)。

By default this file will be the entry point for the test suite and you can add your it()'s there. This will also become bundled so that node env support happens as well.
默认情况下，这个文件将是测试套件的入口点，你可以在那里添加你的it()。
它也会被打包，以便node也能支持env。

### configCases (ConfigTestCases.basictest.js) 1
If you are trying to solve a bug which is reproducible when x and y properties are used together in a config, then configCases is the place to be!!!!
如果你正在尝试解决一个bug，而这个bug在配置文件中同时使用x和y属性时是可以重现的，那么configCases应该放在!!!!

In addition to an index.js, these configCases require a webpack.config.js is located inside of your test suite. This will run this specific config through webpack just as you were building individually. They will use the same loading/bundling technique of your it() tests, however you now have a more specific config use cases that you can write even before you start coding.
除了index.js，这些配置案例还需要在测试套件中添加webpack.config.js文件。
这将通过webpack运行特定的配置，就像你单独构建一样。
它们将使用与it()测试相同的加载/打包技术，但是现在您可以在开始编码之前编写更具体的配置用例。

### statsCases (StatsTestCases.basictest.js)
Stats cases are similar to configCases except specifically focusing on the expected output of your stats. Instead of writing to the console, however the output of stats will be written to disk.

By default, the "expected" outcome is a pain to write by hand so instead when statsCases are run, runner is checking output using jest's awesome snapshot functionality.

Basically you don't need to write any expected behaviors yourself. The assumption is that the stats output from your test code is what you expect.

Please follow the approach described below:

write your test code in statsCases/ folder by creating a separate folder for it, for example statsCases/some-file-import-stats/index.js
import("./someModule");
don't forget the webpack.config.js
run the test
jest will automatically add the output from your test code to StatsTestCases.test.js.snap and you can always check your results there
Next time test will run -> runner will compare results against your output written to snapshot previously
You can read more about SnapShot testing right here

Questions? Comments?
If you are still nervous or don't quite understand, please submit an issue and tag us in it, and provide a relevant PR while working on!

Footnotes
1 webpack's parser supports the use of ES2015 features like arrow functions, harmony exports, etc. However as a library we follow Node.js' timeline for dropping older versions of node. Because of this we expect your tests on GitHub Actions to pass all the way back to NodeJS v10; Therefore if you would like specific tests that use these features to be ignored if they are not supported, then you should add a test.filter.js file. This allows you to import the syntax needed for that test, meanwhile ignoring it on node versions (during CI) that don't support it. webpack has a variety of helpful examples you can refer to if you are just starting out. See the ./helpers folder to find a list of the versions.