# Empathetic Code Design

<img src="empathetic_code_design/dove.svg" width="400" height="320" style="display: block; margin: auto;" alt="Peace logo" />

<div class="caption">

> Communicating Clearly In Code

</div>

<div class="presentation_notes">

1. Today's talk is about empathetic code design.
2. The project I will use to demonstrate this is Peace, a framework to create empathetic and forgiving software automation.
3. Peace provides constraints over logic and data types, and common functionality.
4. By using the framework, logic and data types will be shaped to produce an automation tool that is clear in its communication.
5. A user uses that tool to get their work done, with good user experience.

6. Here's a 3 step process to upload a file to the internet.
7. Download the file, create an S3 bucket, upload the file to that bucket.
8. We can use a shell script to execute this process. *(live demo)*
9. We can also use a tool, created using the Peace framework, to do the same thing.
10. `envman` is a tool I created earlier to do this process. *(live demo)*

11. This is a short demonstration of why one would use this framework.
12. When changing any environment, first we want to understand where we are.
13. Second, we also want to know, where we are headed.
14. Third, we want to know the cost of getting there.
15. Finally, when we understand enough about the change, we can decide to do it.

16. Now, when I say clear communication, it should be quite easy to see that A B C here maps to 1 2 3 there.
17. By presenting information in the right form, at the right level of detail, it's much easier to map between the concept and what is seen.
18. This is how Peace takes care of the user.
19. Today, we are going to look at how Peace takes care of the automation tool developer.
20. How clearly can we communicate between the mental model of a process, and code.

21. When designing a flow, we need to define the items in the flow, and the ordering between those items.
22. The reason these are called "items" instead of "steps", is because in Peace, the emphasis is on the outcome, rather than the actions.
23. Said another way, the purpose of technological processes is the end state, not the work undertaken to get there.
24. That statement does not apply to other kinds of processes, where the work is meaningful.
25. In code, adding items and ordering looks like this.

26. This shows linear ordering; it is possible to have work done in parallel when there is no dependency between items.
27. So here we can adjust the ordering such that A and B are both predecessors of C. A comes before C, and B comes before C.
28. So A and B can both begin at the same time, reducing the overall duration.
29. We'll adjust this in the example, I want you to see that both the file download and the s3 bucket turn blue, and when both are done, then the upload begins. We'll slow down our connection so it is easier to see. *(live demo)*
30. Try doing that in a shell script.

31. After a developer defines the items in a flow, they also need to define how data passes through these items.
32. If you've written automation using bash scripts or YAML, usually data is just strings.
33. For the non-devs among us, using "strings" everywhere is like saying, "I took stuff and did stuff, and here is stuff".
34. It would be much better to say, "I grab my camera, and took a shot, and here's a picture."
35. To better communicate with developers, Peace uses type safety.

36. Because Peace is a framework, each item's automation logic should be shareable.
37. This means, the input to each item, and its output, are API.
38. In Peace, we call the input "parameters", and we use call the output "state".
39. For a file download item, the parameters are the source URL to download from and the path to write to.
40. and the state, can be either "the file's not there", or "it's there, and here is its content hash".

41. Given each item defines its parameters and state, we need a way for developers to pass values into the parameters.
42. Usually, at the start of a process, we know the values of some parameters, and we can pass them straight in.
43. However, sometimes we need part of the process to be completed, to have the information to pass to a subsequent item.
44. For example, if we had a server instead of an S3 bucket, we cannot know what its IP address will be until we the server is created.
45. But we still want to encode into the flow, that the state from an earlier item, is needed to determine the parameter value for a subsequent item.

46. For the simple case, where we know the parameter values up front, we can just plug them in.
47. But we want to be able to express, "get the state from step two, and pass it into step three".
48. So I came up with this, for every item's parameters, the framework will generate a specification type that records how to figure out what values to use.
49. and you can pass it direct values, or you can tell it "here's what states to read, and here's how to extract the value to use for this field".
50. Here's how it looks like in code.

51. Yes, we can read states from multiple items, and use them to compute a value.
52. Best of all, it's type safe -- so within the code you can ask "what parameters do I need to pass in, and get them right".
53. If you make a mistake, you have a really big safety net called the compiler, that will save you.
54. The error message needs improvement, though I haven't yet figured that out.
55. If the parameter type changes, such as when you upgrade your item version, then you get a compilation error, which means you don't have to work to discover where to change -- the compiler *tells you*.

56. Now that we have our logic and our data, we need a way to interact with it.
57. Peace is designed to separate the automation logic and data, from how it is interacted with and presented.
58. Whether it is interactive shell, uninteractive shell, web API, or web page, there is no rendering code in any item.
59. This allows automation developers to use different front ends to the same automation backend.
60. In code, the developer can choose which output to use.

61. Currently the only user facing output implementation is the `CliOutput`, which you've seen.
62. There are a number of other output implementations used in tests,
63. such as the `NoOpOutput` which discards all information, and the `FnTrackerOutput` which stores what output methods were called, with what parameters.
64. Developers can choose to write their own output implementation as well, but
65. It is within the scope of Peace provide implementations for common use cases.

66. So far, we've seen Items, Parameters, Specifications, and Output.
67. As a developer, this should be sufficient to run some commands.
68. First we group all of the above into a `CmdCtx`, then we call the command we want to run for the flow.
69. `StatesSavedReadCmd` and `EnsureCmd` are provided by the framework.
70. The real code still contains code to format the output, but this is something that the framework can provide.

71. For any automation software, the user should only need to pass in item parameters once.
72. Commands run using that flow should be able to recall parameters for the user.
73. How can we make this easy for the automation software developer?
74. The easiest thing, is for them to do nothing.
75. So whenever a command context is built, the following happens.

76. Previous parameter specifications will attempt to be loaded from disk.
77. The provided parameter specification will be merged with those.
78. Validation happens, to make sure there is enough information to resolve parameters for all items.
79. The merged parameter specifications will be serialized to disk.
80. If the parameter specifications don't pass validation, we should get a useful error message.

81. When a command is run, Peace stores information about that execution.
82. It stores this in a `.peace` directory within a workspace directory.
83. The automation tool developer needs to tell Peace how to determine the workspace directory.
84. Because if the tool is run in a sub directory, information should still be stored in the workspace directory.
85. If you think about git, if you run `git status` in any subdirectory of a repository, it will still find the `.git` directory somewhere up the chain.

86. One undeniable way to tell if software works, is to run it.
87. And to tell if it will work in production, we usually want to run it in a replica environment first.
88. Out of the box, Peace provides support for logically separate environments, through profiles.
89. So you can have multiple environments under different names, just like git branching.
90. Let's see a demo.

91. How do we communicate clearly in code?
92. We make sure the API matches the mental model.
93. We make sure we have good error messages.
94. When we require something of the developer, make it easy for them to provide it.
95. and when we adhere to these constraints, it will improve the automation development experience.

</div>
