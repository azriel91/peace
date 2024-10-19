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
    <div style="font-size: 1.5em;">October 2024</div>
</div>


<div class="hidden">

### Notes

1. Heya everyone, my name is Azriel, and today I'll be showing you my automation side project, called Peace.

2. Peace is a framework to create user friendly automation.
3. It provides a set of constraints and common functionality, that when you write code according to these constraints, and plug them into the common functionality, you get a nice tool.
4. As with every side project, there is an origin story.

5. In my first job, a team of us were given the task of fully automating our solution deployment.
6. So this was the deployment process, it's all manual.

7. and, obviously this is the solution. Genius.

8. We wanted this process: click, wait, done.
9. And through engineering eyes, we aimed for:. *(linking: Dimensions)*
10. End-to-end automation.
11. Repeatable correctness.
12. and Performance.
13. and we delivered!

14. If you measure success using these metrics, it was undeniable.
15. We reduced the deployment duration down from 2 weeks of manual steps, to 30 minutes.
16. However, our users said "we hate this!". Azriel, this doesn't enhance our lives.

17. What we really did when we introduced automation, was this.
18. When switching from manual steps to automation, the work changes from doing each step at your own pace, to setting up the parameters for all of the steps, pressing go, and waiting.
19. When it's done, you check if your parameters were correct.
20. If they're correct, that's fine.
21. If they weren't, then you had to understand the error, figure out which step it came from, and which parameters feed into that step.
22. Then when the parameter is fixed, which may take 30 seconds, they still had to wait 30 minutes to confirm if their fix worked,
23. and this delayed feedback loop was frustrating.
24. For new users, it was especially painful:
25. We're telling them to fill in parameters that they don't understand,
26. to feed into a process that they cannot see,
27. to create an environment, that they cannot visualize.
28. So they may not have understood what they were doing, but it was certainly our fault.
29. We created pain.

30. We had engineering eyes, but not human eyes:
31. We took away understandability,
32. We took away control.
33. And when you take away understanding and control, you inadvertently also take away morale. *What little they had.*

34. Ideally we should have built something that provides the benefits of automation,
35. while retaining the benefits of manual execution.
36. This is what the Peace framework aims to do.
37. And today I'd like to show you how it does this, through a tool built using the Peace framework.

38. It's called `envman`, short for environment manager.
39. `envman` automates the download of a web application from github, creates some resources in Amazon, and uploads the web application.
40. Notably there's a missing step to launch a server that runs the web application, but I'm all out of AWS credits. So if you have spare, I'd gladly take them.

41. The first thing we took away was understandability, so let's put that back.

42. There are two ways we tend to write automation:
43. Either we produce too little information, and we can't tell what's going on,
44. or, we produce too much information, and we still can't tell what's going on.
45. For understandability, we need to have something in between.

46. Let's take a look.
47. This is what it looks like when you have too little information:
48. `clear`, `./envman deploy --format none`, clean.
49. Something's going on, I promise!
50. This is what it looks like when you have too much information:
51. `./envman deploy --format json`, clean.
52. And finally, something in between.
53. See if you can see how many steps there are in this process, and whether they complete successfully:
54. `clear`, `./envman deploy`.
55. How many steps are there? *gesture*
56. Did every step complete successfully?
57. "Green means good", so I believe so.
58. What resources were created? *gesture*
59. And if we clean up the environment, you'll see a similar interface, so you can tell that each resource is deleted: `./envman clean`.
60. That's all good when things go well, but what happens in a failure? Can we understand it?
61. First we'll limit the connection speed of the tool to 40 kilobits per second:

    ```sh
    New-NetQosPolicy `
      -Name "envman" `
      -AppPathNameMatchCondition "envman.exe" `
      -PolicyStore ActiveStore `
      -ThrottleRateActionBitsPerSecond 40KB
    ```

62. and run the deployment again: `clear`, `./envman deploy`.
63. You can see that our download from github has slowed,
64. and in a little while we should see an error happen.
65. Here we go.
66. With fresh eyes, can you see which step went wrong?
67. Red means bad, so it should be apparent.
68. In detail, what went wrong, why it went wrong, and how to recover, are all shown.
69. We failed to upload the object. Why? The upload timed out, and make sure you are connected to the internet and try again.
70. We're also shown which resources exist and which don't, so we don't have to guess.
71. If we fix our connection, and re-run the automation:

    ```sh
    Remove-NetQosPolicy `
      -Name "envman" `
      -PolicyStore ActiveStore `
      -Confirm:$false
    ```

72. You'll see that it picks up where it left off, and completes the process.
73. That is, what you think it should do, it does. No surprises.

74. So in summary, with information, the goldilocks principle applies:
75. Too much information is overwhelming, too little is not useful, and there's some middle ground which is just right.
76. The Peace framework generally tries to fit the most relevant information on one screen.

77. The second thing we took away, was control.

78. Most automation tools give you one button -- start -- and that's it.
79. Start the creation, or update, and start the deletion.
80. While pressing start is not difficult, knowing whether the automation will do what we think it will, before we press start, is difficult.

81. What we should understand *before* starting anything, is:
82. Where we are -- our current state,
83. Where we want to go -- our goal state, and
84. The distance between the two.
85. Because if we start with nothing, and end up with something, the distance is something.
86. And if we start with something, and our goal state is something, the distance is nothing.
87. And if we start with something, and our goal state is something else, the distance is that *else*
88. When we understand these three things, then we can make an informed decision if we should press go.
89. Now, if we press start, and change our mind, can we stop the process?
90. Without automation, we can.
91. Like, if someone said, "Azriel! Stop work."
92. I'd say, "Gladly." I can stop where I am.

93. With automation, you need to intentionally build interruptibility into the process.
94. And while pressing Ctrl C on a command line tool is one form of interruption,
95. what we really care about, is safe interruption.
96. i.e. Stop what you're doing when it is safe to do so.
97. Maybe we're at step 5 of a 10 step process, and we want to adjust the parameter for step 7.
98. If we can interrupt the process, adjust the parameters, press go, and have the automation pick up where it left off, that would be great.
99. As in, don't undo all of the work you've already done to get to this point.
100. I just want to fix the parameter for the later step, and continue.

101. Let's see all of this control, in action.
102. Before we run our deployment, what is our environment's current state.
103. Just like we can run `git status`, we can also run `./envman status`.
104. What state will the automation bring our environment to, when we run it? `./envman goal`
105. What's the difference? `./envman diff`
106. The commands are intentionally similar to `git` commands so we make use of familiar names.
107. And for interruptibility, when we deploy, we'll stop the process halfway.
108. `./envman deploy`, ctrl c.

109. Here you can see steps 1 through 3, and step 5 were complete,
110. and step 4 and 6 were not started due to the interruption.
111. If we look at the diff: `./envman diff`,
112. you can see that steps 1, 2, 3, and 5 are done, steps 4 and 6 haven't been executed.
113. If we change our parameters, to using version 0.1.2 instead of 0.1.1 of our web application,
114. the diff will now show that step 1 will change.
115. And if we run deploy again, that is exactly what happens.

116. When cleaning up, we can also interrupt the process.
117. Steps 1, 4, 5, and 6 were cleaned, and 2 and 3 were not.
118. And we can choose to either deploy the environment again, or clean up fully.
119. Let's deploy it to completion. `deploy`, `clean`.

120. What's the use of this?
121. Well there was once we were told,
122. "hey this customer doesn't need their environment anymore, you can delete it."
123. "You sure?"
124. "Yes."
125. So we started the deletion process, and we got this "Hey stop. Stop what you're doing."
126. "We can't. It's all just going to go."
127. And that was the beginning of a very exciting day.
128. So, build a stop button into your automation people.
129. If you use Peace, it is built in for you.

130. We've given back to the user some control, but there are other things still to be implemented like running a subset of the process.
131. Not too hard to implement, just needs time.

132. Morale.
133. Not everyone who uses automation tools has a software background, and not everyone uses the command line all the time.
134. So why not create something that caters for these situations as well?

135. Back to understandability, normally when explaining what automation does,
136. we tend to draw a diagram on the whiteboard,
137. or create a diagram in an internal documentation site.
138. However, it's never really accurate, and it's usually a tangle of overlapping boxes and lines,
139. so it is hard to understand, because the information isn't clear.

140. So here's a web interface. `./envman web`
141. Based on the code written for your automation, two diagrams are generated:
142. The one on the left is called the Progress diagram, which shows the steps in your process,
143. and the one on the right is the Outcome diagram, which shows what the deployed environment looks like, before you deploy it.
144. By clicking on these steps on the right, we get to see what is happening in that step.

145. For example the first step is to download a file from Github, it shows you the request to github and where it saves the file on the file system.
146. Then it creates the IAM policy, role, and instance profile, and S3 bucket,
147. then uploads the web application to that bucket.
148. All of this is generated from your automation code. Magic.
149. This is what you can use to teach someone, or self learn, what the automation process is, and what the environment looks like.
150. And you don't have to keep erasing and redrawing lines on the whiteboard.
151. Which step was unclear? This one? Let's go through that again.

152. Now, this is great, but I like this one.
153. The diagram you saw is the example environment, but what does the actual environment look like?
154. We can discover it.
155. The diagram on the right has faded boxes for each resource, indicating that it doesn't exist.
156. When I click deploy, you can watch the progress diagram on the left, which will show you which steps are being executed,
157. or you can watch the outcome diagram on the right, which will show you the interactions between hosts, that are happening in real time.

158. All of the steps completed successfully, that why they're green,
159. and the resources have been created, so they are now visible.
160. We can do the same for clean up, and it will delete all of the resources from Amazon, as well as on disk.

161. And if we were to have an error, as we did before, we should see it clearly.
162. *slow down internet, click deploy*
163. Let's take a moment to admire this diagram.
164. Ooh look it's gone red.
165. So very quickly, from the user interface, you can tell which step the error came from,
166. as well as which resources it involves.
167. And we can surface the timeout message on the web interface, I just haven't coded that part yet.
168. Cool.

169. So for morale, a lot of effort has been put into aesthetics.
170. For seeing the state of the system, showing one line for each resource, with a link to the full detail, is deliberate.
171. If you've ever been on-call and gotten a call out in the middle of the night, it's very annoying to have to go and find each resource that is part of the system you are investigating.
172. If I can think it, take me there.

173. For progress, we present the information at a level of detail that is digestable,

174. and for errors, instead of panicking, which is visually equivalent of printing a stack trace,
175. we take that error, refine it, and make it beautiful.
176. Always include what went wrong, the reason, and how to recover,
177. because when help people recover from a bad situation,
178. you recover their morale.

179. With all of these aesthetic refinements, that box, is no longer opaque.
180. It is completely, clear.
181. You can see inside it, you can understand it, and you can control it.

182. How does all of this work?
183. Magic.

184. Architecture, how does it fit together?

185. The Peace framework is categorised into two main parts.
186. The item definition, which is the common shape of logic and data, for anything that is managed by automation, and
187. Common functionality, which works with those shapes to provide command execution and a user interface.
188. Item crates contain the logic and data to automate one thing, and
189. the tool crate connects different items together, and passes them to the common functionality from the Peace framework, to provide automation.
190. These groupings are deliberate, so that you can share and reuse common item logic from the standard package registry,
191. while keeping proprietary values and workflows within your tool.
192. Let's go deeper.

193. Starting with Item.
194. If you think of one step in a process, normally we would write code to *do* the step.
195. But instead of only writing code that does the work of that step,
196. an Item is a collection of functions that interact with the thing that is being automated.
197. What is the current state of the thing I'm managing?
198. What will it be, after the automation logic is executed?
199. What's the difference between these states?
200. What does it look like if it's not there?
201. The actual work logic, and
202. interactions -- what are the hosts, and paths that are involved in this automation.
203. Is it a request to fetch data back in, or is it a push to push data out.
204. This information is used to generate the diagram you saw earlier.

205. An example implementation of this, the File Download.
206. The current state function returns the state of the file on disk -- whether or not it exists.
207. And if it does exist, it also returns the MD5 hash.
208. The goal state function returns the state of the file from the server, because the state of the file on the server, will become the state of the file on disk, when the download is executed.
209. So this would fetch the content-length and etag from the server, as a way to compare with what is on disk locally.
210. Many servers use the MD5 hash of a file as its etag.
211. `state_diff` returns whether the local file has the same hash as the remote file.
212. If it's got a different hash, then we assume we need to download it.
213. `state_clean` returns "the file does not exist".
214. `apply` downloads the file.
215. and `interactions` says I'm pulling data from this host, and writing to this path on localhost.

216. A collection of functions is called an Item.
217. And a collection of items, is called a Flow.
218. And a flow also contains the dependency ordering between items.
219. And in Rust, since we cannot store different concrete types in a collection, we have to put them on the heap and store their addresses.
220. Then this flow is what is passed to Peace's common functionality to use in execution or display.

221. Commands. Commands are one of the common functionality that Peace provides.
222. Given a flow and parameters, it invokes different functions within each item.
223. For example, the Discover command.
224. What is the current state of each item? What is the goal state of each item?
225. The discover command will run these functions, store the state, and display it to the user.
226. The Diff command will compute and show the difference between the current and goal states of each item.
227. The Ensure command will turn the current state of each item, into its goal state, through the apply function.
228. The Clean command is similar, where it turns the current state into the clean state, also through the apply function.
229. So Peace provides common logic to iterate through the items, and call the appropriate functions.
230. and it will also pass the appropriate values between each item.
231. That, is magic.

232. Going back to the Item definition, besides the functions to read from or write to the item, implementors also have to specify these data types.
233. Input, which we call parameters, and
234. Output, which we call State.
235. The parameters tell the item where to fetch data from, and where to write to, as well as any other information needed to access the item.
236. The state indicates whether or not the item exists, where it lives, and a summary of its contents.
237. This is the type that is returned from the current, goal, and clean state functions.

238. Putting it all together:
239. We combine the items into a flow,
240. We specify the parameters for each item,
241. Pick an output -- the command line, or web, or both,
242. and these three things together is called a command context.
243. Essentially "all the things you need to run a command".
244. Surface the commands to the user with appropriate names,
245. and this is your tool.

246. So Peace is a side project, and
247. there are side-side projects that were built in the making of Peace.

248. The first noticeable one is Interruptible, which adds the ability to interrupt a stream.
249. If you think about playing music, we are streaming bytes to a speaker, and out comes some audio.
250. When we pause the music, the bytes that were buffered still play, but any bytes that were not the audio buffer will not be played.
251. In automation, instead of streaming bytes to a speaker, we are streaming logic, to an executor.
252. When we pause, any logic that was already queued and is executing, will continue to run to completion.
253. Any logic that hasn't been queued, will not be started.
254. So we fully execute what is in progress to completion, and safely stop between steps.
255. And that's how we get safe interruptibility.

256. The second noticeable project is Dot Interactive.
257. This generates diagrams from structured input.
258. So it takes a data model with the nodes and edges, generates a diagram using GraphViz dot, and adds styles using Tailwind.
259. And that's what I've used for most of the diagrams you've seen today.

260. Now rounding off, what's the status of Peace? Is it ready to be used?

261. For development workflows, or short lived environments, where the environment does not live longer than one version of a tool,
262. I'd say it is ready.
263. But for production workflows, or environments that need to be stable, then Peace is not ready.
264. Don't use it, you will not have Peace.

265. In the table below, you can see the command execution and CLI functionality is stable,
266. The web interface is definitely not stable -- it was hacked together last month for this demo, and
267. the most important one for readiness is API and data stability, which may take me a year to complete.

268. Links to the project:
269. peace.mk for the project website
270. Slides are on peace.mk/book.
271. github.com/azriel91/peace for the repository.

272. To wrap up, I'd like to end with this note:
273. To engineer with empathy,
274. whether it is verbal, visual, or vocal,
275. refine your voice, connect,
276. and communicate with clarity.

277. Thank you for listening, and I'm happy to take questions.

</div>
