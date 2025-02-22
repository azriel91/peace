<div style="
    display: flex;
    flex-wrap: wrap;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    height: 80vh;
">
    <div>
        <object
            type="image/svg+xml"
            data="peace_zero_stress_automation/dove.svg"
            width="450"></object>
        <br />
    </div>
    <div style="font-size: 3.0em; font-weight: bold;">Peace</div>
    <div style="font-size: 2.0em;">Zero Stress Automation</div>
    <div style="font-size: 1.5em;"><a href="https://peace.mk/">https://peace.mk/</a></div>
    <div style="height: 50px;"></div>
    <div style="font-size: 2.0em;">Azriel Hoh</div>
    <div style="font-size: 1.5em;">February 2025</div>
</div>


<div class="hidden">

### Building `envman` in Nushell

```nu
# in $nu.config-path
# usually C:\Users\$env.USER\AppData\Roaming\nushell\config.nu
def envman_demo_prepare_release [] {
    cargo envman_build_release
    let envman_demo_dir = [$env.TEMP demo envman] | path join
    if not ($envman_demo_dir | path exists) {
        mkdir $envman_demo_dir
    }

    let envman_exe_path = [target release envman.exe] | path join
    cp -f $envman_exe_path $envman_demo_dir
    echo $"Copied ($envman_exe_path) to ($envman_demo_dir)"

    let envman_pkg_dir = [target web envman pkg] | path join
    cp -f --recursive $envman_pkg_dir $envman_demo_dir
}

# then
envman_demo_prepare_release
cd ([$env.TEMP demo envman] | path join)
```


### Notes

1. Heya everyone, my name is Azriel, and today I'll be showing you my automation side project, called Peace.

2. Peace is a framework to create zero stress automation.
3. As with every side project, there is an origin story.

4. In my first job, a team of us were given the task of fully automating our solution deployment.
5. So this was the deployment process, it's all manual.

6. and, obviously this is the solution. Genius.

7. We wanted this process: click, wait, done.
8. Being good engineers, we aimed for:. *(linking: Dimensions)*
9. End-to-end automation.
10. Repeatable correctness.
11. and Performance.
12. and we delivered!

13. If you measure success using these metrics, it was undeniable.
14. We reduced the deployment duration down from 2 weeks, to 30 minutes.
15. However, our users said "we hate this!". Azriel, this doesn't enhance our lives.

16. What we really did when we introduced automation, was this.
17. When switching from manual steps to automation, the work changes from doing each step at your own pace, to setting up the parameters for all of the steps, pressing go, and waiting.
18. If it didn't work, then you had to clean up and start again.
19. And for people who were unfamiliar with the process, it was especially painful:
20. We're telling them to fill in parameters that they don't understand,
21. to feed into a process that they cannot see,
22. to create an environment, that they cannot visualize.
23. So they may not have understood what they were doing, but it was certainly our fault.
24. We created pain.

25. We had engineering eyes, but not human eyes:
26. We took away understandability and control.
27. And when you take those away, you inadvertently also take away morale. *What little they had.*

28. Ideally we should have built something that provides the value of automation,
29. while retaining the value of manual execution.
30. This is what the Peace framework aims to do.

31. Normally when we write automation, we spend just enough effort to get things working.
32. I wanted a framework which, by spending that same amount of effort, I would get a much nicer tool.

33. So today I'd like to show you `envman`, a tool built using the Peace framework.
34. `envman` downloads a web application from github, creates some resources in Amazon, and uploads the web application.
35. Notably there's a missing step to launch a server that runs the web application, but that's not free.

36. The first thing we took away was understandability, so let's put that back.

37. There are two ways we tend to write automation:
38. Either we produce too little information, and we can't tell what's going on,
39. or, we produce too much information, and we still can't tell what's going on.
40. For understandability, we need to have something in between.

41. This is what it looks like when you have too little information:
42. `clear`, `./envman deploy --format none`, clean.
43. This is what it looks like when you have too much information:
44. `./envman clean --format json`, clean.
45. The right balance is somewhere in between.
46. `clear`, `./envman deploy`.
47. How many steps are there in this process? *gesture*
48. Did it work?
49. "Green means good", so of course!
50. What resources were created? *gesture*
51. And if we clean up the environment, you'll see a similar interface, so you can tell that each resource is deleted: `./envman clean`.

52. That's all good when things go well, but what happens in a failure? Can we understand it?
53. First we'll limit the connection speed of the tool to 40 kilobytes per second:

    ```sh
    New-NetQosPolicy `
      -Name "envman" `
      -AppPathNameMatchCondition "envman.exe" `
      -PolicyStore ActiveStore `
      -ThrottleRateActionBitsPerSecond 40KB
    ```

54. and run the deployment again: `clear`, `./envman deploy`.
55. You can see that our download from github has slowed,
56. and in a little while we should see an error happen.
57. Here we go.
58. Can you see which step went wrong?
59. Red means bad, so that one.
60. In detail, what went wrong, why it went wrong, and how to recover, are all shown.
61. We failed to upload the object. Why? The upload timed out, and make sure you are connected to the internet and try again.
62. We're also shown which resources exist and which don't, so we don't have to guess.
63. If we fix our connection, and re-run the automation:

    ```sh
    Remove-NetQosPolicy `
      -Name "envman" `
      -PolicyStore ActiveStore `
      -Confirm:$false
    ```

64. You'll see that it picks up where it left off, and completes the process.
65. What you think it should do, it does. No surprises.

66. So in summary, with information, the goldilocks principle applies:
67. Too much information is overwhelming, too little is not useful, and there's some middle ground which is just right.
68. The Peace framework generally tries to fit the most relevant information on one screen.

69. The second thing we took away, was control.

70. Most automation tools give you one button -- start -- and that's it.
71. While pressing start is not difficult, knowing whether the automation will do, what we think it will, is difficult.

72. What we should understand *before* starting anything, is:
73. Where we are -- our current state,
74. Where we want to go -- our goal state, and
75. The distance between the two.
76. Then we can press start.

77. But when we press start, and change our mind, can we stop the process?
78. Without automation, we can.

79. With automation, while pressing Ctrl C on a command line tool is one form of interruption,
80. what we really care about, is safe interruption.
81. If we can interrupt the process, adjust the parameters, press go, and have the automation pick up where it left off, that would be great.
82. And don't undo all of the work you did to get to this point.

83. Let's see all of this control, in action.
84. Before we run our deployment, what is the current state? `./envman status`.
85. Target state? `./envman goal`
86. What's the difference? `./envman diff`
87. And for interruptibility, when we deploy, we'll stop the process halfway.
88. `./envman deploy`, ctrl c.

89. Here you can see steps 1 through 3, and step 5 were complete,
90. and step 4 and 6 were not started due to the interruption.
91. If we look at the diff: `./envman diff`,
92. you can see that steps 1, 2, 3, and 5 are done, steps 4 and 6 haven't been executed.
93. If we change our parameters, to using version 0.1.2 instead of 0.1.1 of our web application,
94. the diff will now show that step 1 will change.
95. And if we run deploy again, that is exactly what happens.

96. When cleaning up, we can also interrupt the process.
97. Steps 1, 4, 5, and 6 were cleaned, and 2 and 3 were not.
98. And we can choose to either deploy the environment again, or clean up fully.
99. Let's deploy it to completion. `deploy`, `clean`.

100. Morale.
101. Not everyone who uses automation tools has a software background, and not everyone uses the command line all the time.
102. So why not create something that caters for these situations as well?

103. Back to understandability, normally when explaining what automation does,
104. we tend to draw a diagram on the whiteboard,
105. or create a diagram in an internal documentation site.
106. However, it's never really accurate, and it's usually a tangle of overlapping boxes and lines,
107. so it is hard to understand, because the information isn't clear.

108. So here's a web interface. `./envman web`
109. Based on the code written for your automation, two diagrams are generated:
110. The one on the left is called the Progress diagram, which shows the steps in your process,
111. and the one on the right is the Outcome diagram, which shows what the deployed environment looks like, before you deploy it.
112. By clicking on these steps on the right, we get to see what is happening in that step.

113. All of this is generated from your automation code. Magic.
114. This is what you can use to teach someone, or self learn, what the automation process is, and what the environment looks like.
115. And you don't have to keep erasing and redrawing lines on the whiteboard.
116. Which step was unclear? This one? Let's go through that again.

117. Now, this is great, but I like this one.
118. The diagram you saw is the example environment, but what does the actual environment look like?
119. We can discover it.
120. The diagram on the right has faded boxes for each resource, indicating that it doesn't exist.
121. When I click deploy, you can watch the progress diagram on the left, which will show you which steps are being executed,
122. or you can watch the outcome diagram on the right, which will show you the interactions between hosts, that are happening in real time.

123. All of the resources have been created, so they are now visible.
124. When we clean up, the boxes in the diagram become faded again.

125. And if we were to have an error, as we did before, we should see it clearly.
126. *slow down internet, click deploy*
127. Hey look it's gone red.
128. So very quickly, from the user interface, you can tell which step the error came from,
129. as well as which resources it involves.
130. And we can surface the timeout message on the web interface, I just haven't coded that part yet.
131. Cool.

132. So for morale, a lot of effort has been put into aesthetics.
133. For seeing the state of the system, showing one line for each resource, with a link to the full detail, is deliberate.
134. If you've ever been on-call and gotten a call out in the middle of the night, it's very annoying to have to go and find each resource that is part of the system you are investigating.
135. If I can think it, take me there.

136. For progress, we present the information at a level of detail that is digestable,

137. and for errors, instead of panicking, which is visually equivalent of printing a stack trace,
138. we take that error, refine it, and make it beautiful.
139. Always include what went wrong, the reason, and how to recover,
140. because when help people recover from a bad situation,
141. you recover their morale.

142. With all of these aesthetic refinements, that box, is no longer opaque.
143. It is completely, clear.
144. You can see inside it, you can understand it, and you can control it.

145. How does all of this work?
146. Magic.

147. Architecture, how does it fit together?

148. The Peace framework is categorised into two main parts.
149. The item definition, which is the common shape of logic and data, for anything that is managed by automation, and
150. Common functionality, which works with those items to provide command execution and a user interface.
151. Item crates contain the logic and data to automate one thing, and
152. a tool is the thing that connects items together and passes them to the Peace framework.
153. These groupings are deliberate, so that you can share and reuse common automation logic,
154. while keeping proprietary values and workflows within your tool.
155. Let's go deeper.

156. If you think of one step in a process, normally we would write code to *do* the step.
157. But instead of only writing code that does work,
158. we also have functions that fetch information about the step.
159. What is the current state of the thing I'm managing?
160. What will it be, after the automation logic is executed?
161. What's the difference between these states?
162. What does it look like if it's not there?
163. Essentially, functions to show me what it is and what it will be, without changing anything.

164. A collection of functions is called an Item.
165. And a collection of items, is called a Flow.
166. And a flow also contains the dependency ordering between items.
167. Then this flow is passed to Peace's to execute or display information.

168. Commands. Commands are one of the common functionality that Peace provides.
169. Given a flow and parameters, it invokes different functions within each item.
170. For example, the Discover command will run these functions, store the state, and display it to the user.
171. The Diff command will compute and show the difference between the current and goal states of each item.
172. The Ensure command will turn the current state of each item, into its goal state, through the apply function.
173. The Clean command is similar, where it turns the current state into the clean state, also through the apply function.
174. So Peace provides common logic to iterate through the items, and call the appropriate functions.
175. and it will also pass the appropriate values between each item.
176. That, is magic.

177. Putting it all together:
178. We combine the items into a flow,
179. We specify the parameters for each item,
180. Pick an output -- the command line, or web, or both,
181. and call the right command.
182. Surface the commands to the user with appropriate names,
183. and this is your tool.

184. Now rounding off, what's the status of Peace? Is it ready to be used?

185. For development workflows, or short lived environments, where the environment does not live longer than one version of a tool,
186. It's usable.
187. But for production workflows, or environments that need to be stable, then Peace is not ready.
188. Don't use it, you will not have Peace.

189. Links to the project:
190. peace.mk for the project website
191. Slides are on peace.mk/book.
192. github.com/azriel91/peace for the repository.

193. To wrap up, I'd like to end with this note:
194. To engineer with empathy,
195. whether it is verbal, visual, or vocal,
196. refine your voice, connect,
197. and communicate with clarity.

198. Thank you for listening, and I'm happy to take questions.

</div>
