# Example Intermediate Representation

This is the computed data structure from combining the layered values from the input data.

````yaml
---
# Example Intermediate Representation
#
# This is the computed data from combining the layered values from
# the example input.

# Everything that needs to be represented as a `node` on the diagram, including:
#
# * `things`
# * `tags`
# * `processes`
# * `process_steps`
nodes: &nodes
  #
  # things
  t_aws: "☁️ Amazon Web Services"
  t_aws_iam: "🖊️ Identity and Access Management"
  t_aws_iam_ecs_policy: "🗒️ ECS IAM Policy"
  t_aws_ecr: "🗄️ Elastic Container Registry"
  t_aws_ecr_repo: "💽 web_app repo"
  t_aws_ecr_repo_image_1: "💿 Image 1"
  t_aws_ecr_repo_image_2: "💿 Image 2"
  t_aws_ecs: "💻 Elastic Container Service"
  t_aws_ecs_cluster_app: "🎛️ web_app cluster"
  t_aws_ecs_cluster_app_task: "🖨️ web_app task version 1"
  t_github: "🐙 GitHub"
  t_github_user_repo: "azriel91/web_app"
  t_localhost: "🧑‍💻 localhost"
  t_localhost_repo: "📂 ~/work/web_app"
  t_localhost_repo_src: "📝 src"
  t_localhost_repo_target: "📂 target"
  t_localhost_repo_target_file_zip: "📝 file.zip"
  t_localhost_repo_target_dist_dir: "📁 dist"

  # tags
  tag_app_development: "Application Development"
  tag_deployment: "Deployment"

  # processes and process steps
  proc_app_dev: "App Development"
  proc_app_dev_step_repository_clone: "Clone repository"
  proc_app_dev_step_project_build: "Build project"

  proc_app_release: "App Release"
  proc_app_release_step_crate_version_update: "Update crate versions"
  proc_app_release_step_pull_request_open: "Open PR"
  proc_app_release_step_tag_and_push: "Tag and push"
  proc_app_release_step_gh_actions_build: "Github Actions build"
  proc_app_release_step_gh_actions_publish: "Github Actions publish"

  proc_i12e_region_tier_app_deploy: "Prod App Deploy"
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: "Update ECS service"

# Text to copy to clipboard when a node's copy button is clicked.
node_copy_text:
  # the reordering is *probably* from serde-saphyr's implementation of merge keys.
  t_localhost: "localhost"
  t_localhost_repo: "~/work/web_app"
  t_localhost_repo_src: "~/work/web_app/src"
  t_localhost_repo_target: "~/work/web_app/target"
  t_localhost_repo_target_file_zip: "~/work/web_app/target/file.zip"
  t_localhost_repo_target_dist_dir: "~/work/web_app/target/dist"
  t_aws: "☁️ Amazon Web Services"
  t_aws_iam: "🖊️ Identity and Access Management"
  t_aws_iam_ecs_policy: "🗒️ ECS IAM Policy"
  t_aws_ecr: "🗄️ Elastic Container Registry"
  t_aws_ecr_repo: "💽 web_app repo"
  t_aws_ecr_repo_image_1: "💿 Image 1"
  t_aws_ecr_repo_image_2: "💿 Image 2"
  t_aws_ecs: "💻 Elastic Container Service"
  t_aws_ecs_cluster_app: "🎛️ web_app cluster"
  t_aws_ecs_cluster_app_task: "🖨️ web_app task version 1"
  t_github: "🐙 GitHub"
  t_github_user_repo: "azriel91/web_app"

# Hierarchy of all nodes.
node_hierarchy:
  # Tags before everything else
  tag_app_development: {}
  tag_deployment: {}

  # Processes before things/edges
  proc_app_dev:
    proc_app_dev_step_repository_clone: {}
    proc_app_dev_step_project_build: {}
  proc_app_release:
    proc_app_release_step_crate_version_update: {}
    proc_app_release_step_pull_request_open: {}
    proc_app_release_step_tag_and_push: {}
    proc_app_release_step_gh_actions_build: {}
    proc_app_release_step_gh_actions_publish: {}
  proc_i12e_region_tier_app_deploy:
    proc_i12e_region_tier_app_deploy_step_ecs_service_update: {}

  # Things
  t_aws:
    t_aws_iam:
      t_aws_iam_ecs_policy: {}
    t_aws_ecr:
      t_aws_ecr_repo:
        t_aws_ecr_repo_image_1: {}
        t_aws_ecr_repo_image_2: {}
    t_aws_ecs:
      t_aws_ecs_cluster_app:
        t_aws_ecs_cluster_app_task: {}
  t_github:
    t_github_user_repo: {}
  t_localhost:
    t_localhost_repo:
      t_localhost_repo_src: {}
      t_localhost_repo_target:
        t_localhost_repo_target_file_zip: {}
        t_localhost_repo_target_dist_dir: {}

# Edge groups with explicit from/to edges.
edge_groups:
  # Dependency edge groups
  edge_dep_t_localhost__t_github_user_repo__pull:
    # cyclic: [t_localhost, t_github_user_repo]: A -> B -> A
    - from: t_localhost
      to: t_github_user_repo
    - from: t_github_user_repo
      to: t_localhost
  edge_dep_t_localhost__t_github_user_repo__push:
    # sequence: [t_localhost, t_github_user_repo]: A -> B
    - from: t_localhost
      to: t_github_user_repo
  edge_dep_t_localhost__t_localhost__within:
    # cyclic: [t_localhost]: A -> A
    - from: t_localhost
      to: t_localhost
  edge_dep_t_github_user_repo__t_github_user_repo__within:
    # symmetric: [t_github_user_repo] (1 thing special case): A -> A (request), A -> A (response)
    - from: t_github_user_repo
      to: t_github_user_repo
    - from: t_github_user_repo
      to: t_github_user_repo
  edge_dep_t_github_user_repo__t_aws_ecr_repo__push:
    # sequence: [t_github_user_repo, t_aws_ecr_repo]: A -> B
    - from: t_github_user_repo
      to: t_aws_ecr_repo
  edge_dep_t_aws_ecr_repo__t_aws_ecs_service__push:
    # sequence: [t_aws_ecr_repo, t_aws_ecs_service]: A -> B
    - from: t_aws_ecr_repo
      to: t_aws_ecs_service

  # Interaction edge groups
  edge_ix_t_localhost__t_github_user_repo__pull:
    # symmetric: [t_localhost, t_github_user_repo]: A -> B -> B -> A
    - from: t_localhost
      to: t_github_user_repo
    - from: t_github_user_repo
      to: t_localhost
  edge_ix_t_localhost__t_github_user_repo__push:
    # sequence: [t_localhost, t_github_user_repo]: A -> B
    - from: t_localhost
      to: t_github_user_repo
  edge_ix_t_localhost__t_localhost__within:
    # cyclic: [t_localhost]: A -> A
    - from: t_localhost
      to: t_localhost
  edge_ix_t_github_user_repo__t_github_user_repo__within:
    # symmetric: [t_github_user_repo] (1 thing special case): A -> A (request), A -> A (response)
    - from: t_github_user_repo
      to: t_github_user_repo
    - from: t_github_user_repo
      to: t_github_user_repo
  edge_ix_t_github_user_repo__t_aws_ecr_repo__push:
    # sequence: [t_github_user_repo, t_aws_ecr_repo]: A -> B
    - from: t_github_user_repo
      to: t_aws_ecr_repo
  edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push:
    # sequence: [t_aws_ecr_repo, t_aws_ecs_service]: A -> B
    - from: t_aws_ecr_repo
      to: t_aws_ecs_service

# Descriptions for entities.
entity_descs:
  # nodes
  t_localhost: "User's computer"

  # edge groups
  edge_ix_t_localhost__t_github_user_repo__pull: "Fetch from GitHub"
  edge_ix_t_localhost__t_github_user_repo__push: "Push to GitHub"

  # edges
  edge_ix_t_localhost__t_github_user_repo__pull__0: "`git pull`"
  edge_ix_t_localhost__t_github_user_repo__push__0: "`git push`"

  # process_steps
  proc_app_dev_step_repository_clone: |-
    ```bash
    git clone https://github.com/azriel91/web_app.git
    ```
  proc_app_dev_step_project_build: |-
    Develop the app:

    * Always link to issue.
    * Open PR.

  proc_app_release_step_crate_version_update: |-
    ```bash
    sd -s 'version = "0.3.0"' 'version = "0.3.0"' $(fd -tf -F toml) README.md src/lib.rs
    ```
  proc_app_release_step_pull_request_open: |-
    Create a pull request as usual.
  proc_app_release_step_gh_actions_build: |-
    Github Actions will build the image.
  proc_app_release_step_tag_and_push: |-
    When the PR is merged, tag the commit and push the tag to GitHub.

    ```bash
    git tag 0.3.0
    git push origin 0.3.0
    ```

    The build will push the new version to ECR automatically.
  proc_app_release_step_gh_actions_publish: |-
    Github Actions will publish the image to AWS ECR.
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: |-
    Deploy or update the existing ECS service with the new image.

# Entity types for styling.
entity_types:
  # things
  t_aws: [type_thing_default, type_organisation]
  t_aws_iam: [type_thing_default, type_service]
  t_aws_iam_ecs_policy: [type_thing_default]
  t_aws_ecr: [type_thing_default, type_service]
  t_aws_ecr_repo: [type_thing_default]
  t_aws_ecr_repo_image_1: [type_thing_default, type_docker_image]
  t_aws_ecr_repo_image_2: [type_thing_default, type_docker_image]
  t_aws_ecs: [type_thing_default, type_service]
  t_aws_ecs_cluster_app: [type_thing_default]
  t_aws_ecs_cluster_app_task: [type_thing_default]
  t_github: [type_thing_default, type_organisation]
  t_github_user_repo: [type_thing_default]
  t_localhost: [type_thing_default]
  t_localhost_repo: [type_thing_default]
  t_localhost_repo_src: [type_thing_default]
  t_localhost_repo_target: [type_thing_default]
  t_localhost_repo_target_file_zip: [type_thing_default]
  t_localhost_repo_target_dist_dir: [type_thing_default]

  # tags
  tag_app_development: [type_tag_default]
  tag_deployment: [type_tag_default]

  # processes and process steps
  proc_app_dev: [type_process_default]
  proc_app_dev_step_repository_clone: [type_process_step_default]
  proc_app_dev_step_project_build: [type_process_step_default]

  proc_app_release: [type_process_default]
  proc_app_release_step_crate_version_update: [type_process_step_default]
  proc_app_release_step_pull_request_open: [type_process_step_default]
  proc_app_release_step_tag_and_push: [type_process_step_default]
  proc_app_release_step_gh_actions_build: [type_process_step_default]
  proc_app_release_step_gh_actions_publish: [type_process_step_default]

  proc_i12e_region_tier_app_deploy: [type_process_default]
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: [type_process_step_default]

  # edge groups

  # dependency edge groups and edges

  # edge_dep_t_localhost__t_github_user_repo__pull: cyclic [t_localhost, t_github_user_repo]
  #   edges:
  #     - 0: t_localhost -> t_github_user_repo
  #     - 1: t_github_user_repo -> t_localhost
  edge_dep_t_localhost__t_github_user_repo__pull: [type_dependency_edge_cyclic_default]
  edge_dep_t_localhost__t_github_user_repo__pull__0: [type_dependency_edge_cyclic_forward_default]
  edge_dep_t_localhost__t_github_user_repo__pull__1: [type_dependency_edge_cyclic_forward_default]

  # edge_dep_t_localhost__t_github_user_repo__push: sequence [t_localhost, t_github_user_repo]
  #   edges:
  #     - 0: t_localhost -> t_github_user_repo
  edge_dep_t_localhost__t_github_user_repo__push: [type_dependency_edge_sequence_default]
  edge_dep_t_localhost__t_github_user_repo__push__0: [type_dependency_edge_sequence_forward_default]

  # edge_dep_t_localhost__t_localhost__within: cyclic [t_localhost]
  #   edges:
  #     - 0: t_localhost -> t_localhost
  edge_dep_t_localhost__t_localhost__within: [type_dependency_edge_cyclic_default]
  edge_dep_t_localhost__t_localhost__within__0: [type_dependency_edge_cyclic_forward_default]

  # edge_dep_t_github_user_repo__t_github_user_repo__within: symmetric [t_github_user_repo] (1 thing special case)
  #   edges:
  #     - 0: request
  #     - 1: response
  edge_dep_t_github_user_repo__t_github_user_repo__within: [type_dependency_edge_symmetric_default]
  edge_dep_t_github_user_repo__t_github_user_repo__within__0:
    [type_dependency_edge_symmetric_forward_default]
  edge_dep_t_github_user_repo__t_github_user_repo__within__1:
    [type_dependency_edge_symmetric_reverse_default]

  # edge_dep_t_github_user_repo__t_aws_ecr_repo__push: sequence [t_github_user_repo, t_aws_ecr_repo]
  #   edges:
  #     - 0: t_github_user_repo -> t_aws_ecr_repo
  edge_dep_t_github_user_repo__t_aws_ecr_repo__push: [type_dependency_edge_sequence_default]
  edge_dep_t_github_user_repo__t_aws_ecr_repo__push__0:
    [type_dependency_edge_sequence_forward_default]

  # edge_dep_t_aws_ecr_repo__t_aws_ecs_service__push: sequence [t_aws_ecr_repo, t_aws_ecs_service]
  #   edges:
  #     - 0: t_aws_ecr_repo -> t_aws_ecs_service
  edge_dep_t_aws_ecr_repo__t_aws_ecs_service__push: [type_dependency_edge_sequence_default]
  edge_dep_t_aws_ecr_repo__t_aws_ecs_service__push__0:
    [type_dependency_edge_sequence_forward_default]

  # interaction edge groups and edges

  # edge_ix_t_localhost__t_github_user_repo__pull: symmetric [t_localhost, t_github_user_repo]
  #   edges:
  #     - 0: t_localhost -> t_github_user_repo (request)
  #     - 1: t_github_user_repo -> t_localhost (response)
  edge_ix_t_localhost__t_github_user_repo__pull: [type_interaction_edge_symmetric_default]
  edge_ix_t_localhost__t_github_user_repo__pull__0:
    [type_interaction_edge_symmetric_forward_default]
  edge_ix_t_localhost__t_github_user_repo__pull__1:
    [type_interaction_edge_symmetric_reverse_default]

  # edge_ix_t_localhost__t_github_user_repo__push: sequence [t_localhost, t_github_user_repo]
  #   edges:
  #     - 0: t_localhost -> t_github_user_repo
  edge_ix_t_localhost__t_github_user_repo__push: [type_interaction_edge_sequence_default]
  edge_ix_t_localhost__t_github_user_repo__push__0: [type_interaction_edge_sequence_forward_default]

  # edge_ix_t_localhost__t_localhost__within: cyclic [t_localhost]
  #   edges:
  #     - 0: t_localhost -> t_localhost
  edge_ix_t_localhost__t_localhost__within: [type_interaction_edge_cyclic_default]
  edge_ix_t_localhost__t_localhost__within__0: [type_interaction_edge_cyclic_forward_default]

  # edge_ix_t_github_user_repo__t_github_user_repo__within: symmetric [t_github_user_repo] (1 thing special case)
  #   edges:
  #     - 0: request
  #     - 1: response
  edge_ix_t_github_user_repo__t_github_user_repo__within: [type_interaction_edge_symmetric_default]
  edge_ix_t_github_user_repo__t_github_user_repo__within__0:
    [type_interaction_edge_symmetric_forward_default]
  edge_ix_t_github_user_repo__t_github_user_repo__within__1:
    [type_interaction_edge_symmetric_reverse_default]

  # edge_ix_t_github_user_repo__t_aws_ecr_repo__push: sequence [t_github_user_repo, t_aws_ecr_repo]
  #   edges:
  #     - 0: t_github_user_repo -> t_aws_ecr_repo
  edge_ix_t_github_user_repo__t_aws_ecr_repo__push: [type_interaction_edge_sequence_default]
  edge_ix_t_github_user_repo__t_aws_ecr_repo__push__0:
    [type_interaction_edge_sequence_forward_default]

  # edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push: sequence [t_aws_ecr_repo, t_aws_ecs_service]
  #   edges:
  #     - 0: t_aws_ecr_repo -> t_aws_ecs_service
  edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push: [type_interaction_edge_sequence_default]
  edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push__0:
    [type_interaction_edge_sequence_forward_default]

# Computed Tailwind CSS classes.
tailwind_classes:
  # Styles for things are as follows:
  #
  # For the inner `<path>` element:
  #
  # * `hover:stroke-{color}-{shade}` when hovered.
  # * `stroke-{color}-{shade}` for normal state.
  # * `focus:stroke-{color}-{shade}` when focused.
  # * `active:stroke-{color}-{shade}` when pressed.
  # * `hover:fill-{color}-{shade}` when hovered.
  # * `fill-{color}-{shade}` for normal state.
  # * `focus:fill-{color}-{shade}` when focused.
  # * `active:fill-{color}-{shade}` when pressed.
  #
  # For the inner `<text>` element:
  #
  # * `[&>text]:fill-{color}-{shade}` for text in all states.
  #
  # The following classes are used to highlight the `thing`'s `<path>` when a related process step
  # is focused:
  #
  # * `animate-[stroke-dashoffset-move_2s_linear_infinite]`
  # * `stroke-{color}-{shade}` where `shade` is the "normal" shade, but brighter by one level.
  # * `fill-{color}-{shade}` where `shade` is the "normal" shade, but brighter by one level.
  #
  # They are applied to each thing, just with the appropriate `peer-[:focus-within]/{process_step}` prefix.
  #
  # We could collapse the `stroke-dashoffset` into the animation, but we can't collapse the `shade`
  # because they are part of the stroke and fill colors -- we have to choose between duplication
  # either here, or in animations.
  t_aws: |
    visible
    [stroke-dasharray:2]
    stroke-1
    hover:fill-yellow-50
    fill-yellow-100
    focus:fill-yellow-200
    active:fill-yellow-300
    hover:stroke-yellow-100
    stroke-yellow-200
    focus:stroke-yellow-300
    active:stroke-yellow-400
    [&>text]:fill-neutral-800
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_aws_iam: |
    visible
    [stroke-dasharray:3]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_aws_iam_ecs_policy: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_aws_ecr: |
    visible
    [stroke-dasharray:3]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_aws_ecr_repo: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:[stroke-dasharray:3]
    peer-[:focus-within]/tag_deployment:stroke-2
    peer-[:focus-within]/tag_deployment:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/tag_deployment:hover:fill-slate-50
    peer-[:focus-within]/tag_deployment:fill-slate-100
    peer-[:focus-within]/tag_deployment:focus:fill-slate-200
    peer-[:focus-within]/tag_deployment:active:fill-slate-300
    peer-[:focus-within]/tag_deployment:hover:stroke-slate-100
    peer-[:focus-within]/tag_deployment:stroke-slate-200
    peer-[:focus-within]/tag_deployment:focus:stroke-slate-300
    peer-[:focus-within]/tag_deployment:active:stroke-slate-400
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:stroke-2
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:hover:fill-slate-50
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:fill-slate-100
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:focus:fill-slate-200
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:active:fill-slate-300
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:stroke-slate-200
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:active:stroke-slate-400
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:[stroke-dasharray:3]
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:stroke-2
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:hover:fill-slate-50
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:fill-slate-100
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:focus:fill-slate-200
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:active:fill-slate-300
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:hover:stroke-slate-100
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:stroke-slate-200
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:focus:stroke-slate-300
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:active:stroke-slate-400

  t_aws_ecr_repo_image_1: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-200
    fill-sky-300
    focus:fill-sky-400
    active:fill-sky-500
    hover:stroke-sky-300
    stroke-sky-400
    focus:stroke-sky-500
    active:stroke-sky-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_aws_ecr_repo_image_2: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-200
    fill-sky-300
    focus:fill-sky-400
    active:fill-sky-500
    hover:stroke-sky-300
    stroke-sky-400
    focus:stroke-sky-500
    active:stroke-sky-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_aws_ecs: |
    visible
    [stroke-dasharray:3]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_aws_ecs_cluster_app: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_aws_ecs_cluster_app_task: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_github: |
    visible
    [stroke-dasharray:2]
    stroke-1
    hover:fill-neutral-50
    fill-neutral-100
    focus:fill-neutral-200
    active:fill-neutral-300
    hover:stroke-neutral-100
    stroke-neutral-200
    focus:stroke-neutral-300
    active:stroke-neutral-400
    [&>text]:fill-neutral-800
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_github_user_repo: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:[stroke-dasharray:3]
    peer-[:focus-within]/tag_app_development:stroke-2
    peer-[:focus-within]/tag_app_development:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/tag_app_development:hover:fill-slate-50
    peer-[:focus-within]/tag_app_development:fill-slate-100
    peer-[:focus-within]/tag_app_development:focus:fill-slate-200
    peer-[:focus-within]/tag_app_development:active:fill-slate-300
    peer-[:focus-within]/tag_app_development:hover:stroke-slate-100
    peer-[:focus-within]/tag_app_development:stroke-slate-200
    peer-[:focus-within]/tag_app_development:focus:stroke-slate-300
    peer-[:focus-within]/tag_app_development:active:stroke-slate-400
    peer-[:focus-within]/tag_deployment:[stroke-dasharray:3]
    peer-[:focus-within]/tag_deployment:stroke-2
    peer-[:focus-within]/tag_deployment:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/tag_deployment:hover:fill-slate-50
    peer-[:focus-within]/tag_deployment:fill-slate-100
    peer-[:focus-within]/tag_deployment:focus:fill-slate-200
    peer-[:focus-within]/tag_deployment:active:fill-slate-300
    peer-[:focus-within]/tag_deployment:hover:stroke-slate-100
    peer-[:focus-within]/tag_deployment:stroke-slate-200
    peer-[:focus-within]/tag_deployment:focus:stroke-slate-300
    peer-[:focus-within]/tag_deployment:active:stroke-slate-400
    peer-[:focus-within]/proc_app_dev_step_repository_clone:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_dev_step_repository_clone:stroke-2
    peer-[:focus-within]/proc_app_dev_step_repository_clone:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_dev_step_repository_clone:hover:fill-slate-50
    peer-[:focus-within]/proc_app_dev_step_repository_clone:fill-slate-100
    peer-[:focus-within]/proc_app_dev_step_repository_clone:focus:fill-slate-200
    peer-[:focus-within]/proc_app_dev_step_repository_clone:active:fill-slate-300
    peer-[:focus-within]/proc_app_dev_step_repository_clone:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_dev_step_repository_clone:stroke-slate-200
    peer-[:focus-within]/proc_app_dev_step_repository_clone:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_dev_step_repository_clone:active:stroke-slate-400
    peer-[:focus-within]/proc_app_release_step_pull_request_open:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_release_step_pull_request_open:stroke-2
    peer-[:focus-within]/proc_app_release_step_pull_request_open:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_release_step_pull_request_open:hover:fill-slate-50
    peer-[:focus-within]/proc_app_release_step_pull_request_open:fill-slate-100
    peer-[:focus-within]/proc_app_release_step_pull_request_open:focus:fill-slate-200
    peer-[:focus-within]/proc_app_release_step_pull_request_open:active:fill-slate-300
    peer-[:focus-within]/proc_app_release_step_pull_request_open:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_release_step_pull_request_open:stroke-slate-200
    peer-[:focus-within]/proc_app_release_step_pull_request_open:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_release_step_pull_request_open:active:stroke-slate-400
    peer-[:focus-within]/proc_app_release_step_tag_and_push:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_release_step_tag_and_push:stroke-2
    peer-[:focus-within]/proc_app_release_step_tag_and_push:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_release_step_tag_and_push:hover:fill-slate-50
    peer-[:focus-within]/proc_app_release_step_tag_and_push:fill-slate-100
    peer-[:focus-within]/proc_app_release_step_tag_and_push:focus:fill-slate-200
    peer-[:focus-within]/proc_app_release_step_tag_and_push:active:fill-slate-300
    peer-[:focus-within]/proc_app_release_step_tag_and_push:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_release_step_tag_and_push:stroke-slate-200
    peer-[:focus-within]/proc_app_release_step_tag_and_push:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_release_step_tag_and_push:active:stroke-slate-400
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:stroke-2
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:hover:fill-slate-50
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:fill-slate-100
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:focus:fill-slate-200
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:active:fill-slate-300
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:stroke-slate-200
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:active:stroke-slate-400
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:stroke-2
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:hover:fill-slate-50
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:fill-slate-100
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:focus:fill-slate-200
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:active:fill-slate-300
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:stroke-slate-200
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:active:stroke-slate-400

  t_localhost: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:[stroke-dasharray:3]
    peer-[:focus-within]/tag_app_development:stroke-2
    peer-[:focus-within]/tag_app_development:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/tag_app_development:hover:fill-slate-50
    peer-[:focus-within]/tag_app_development:fill-slate-100
    peer-[:focus-within]/tag_app_development:focus:fill-slate-200
    peer-[:focus-within]/tag_app_development:active:fill-slate-300
    peer-[:focus-within]/tag_app_development:hover:stroke-slate-100
    peer-[:focus-within]/tag_app_development:stroke-slate-200
    peer-[:focus-within]/tag_app_development:focus:stroke-slate-300
    peer-[:focus-within]/tag_app_development:active:stroke-slate-400
    peer-[:focus-within]/tag_deployment:opacity-75
    peer-[:focus-within]/proc_app_dev_step_repository_clone:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_dev_step_repository_clone:stroke-2
    peer-[:focus-within]/proc_app_dev_step_repository_clone:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_dev_step_repository_clone:hover:fill-slate-50
    peer-[:focus-within]/proc_app_dev_step_repository_clone:fill-slate-100
    peer-[:focus-within]/proc_app_dev_step_repository_clone:focus:fill-slate-200
    peer-[:focus-within]/proc_app_dev_step_repository_clone:active:fill-slate-300
    peer-[:focus-within]/proc_app_dev_step_repository_clone:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_dev_step_repository_clone:stroke-slate-200
    peer-[:focus-within]/proc_app_dev_step_repository_clone:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_dev_step_repository_clone:active:stroke-slate-400
    peer-[:focus-within]/proc_app_dev_step_project_build:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_dev_step_project_build:stroke-2
    peer-[:focus-within]/proc_app_dev_step_project_build:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_dev_step_project_build:hover:fill-slate-50
    peer-[:focus-within]/proc_app_dev_step_project_build:fill-slate-100
    peer-[:focus-within]/proc_app_dev_step_project_build:focus:fill-slate-200
    peer-[:focus-within]/proc_app_dev_step_project_build:active:fill-slate-300
    peer-[:focus-within]/proc_app_dev_step_project_build:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_dev_step_project_build:stroke-slate-200
    peer-[:focus-within]/proc_app_dev_step_project_build:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_dev_step_project_build:active:stroke-slate-400
    peer-[:focus-within]/proc_app_release_step_crate_version_update:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_release_step_crate_version_update:stroke-2
    peer-[:focus-within]/proc_app_release_step_crate_version_update:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_release_step_crate_version_update:hover:fill-slate-50
    peer-[:focus-within]/proc_app_release_step_crate_version_update:fill-slate-100
    peer-[:focus-within]/proc_app_release_step_crate_version_update:focus:fill-slate-200
    peer-[:focus-within]/proc_app_release_step_crate_version_update:active:fill-slate-300
    peer-[:focus-within]/proc_app_release_step_crate_version_update:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_release_step_crate_version_update:stroke-slate-200
    peer-[:focus-within]/proc_app_release_step_crate_version_update:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_release_step_crate_version_update:active:stroke-slate-400
    peer-[:focus-within]/proc_app_release_step_pull_request_open:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_release_step_pull_request_open:stroke-2
    peer-[:focus-within]/proc_app_release_step_pull_request_open:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_release_step_pull_request_open:hover:fill-slate-50
    peer-[:focus-within]/proc_app_release_step_pull_request_open:fill-slate-100
    peer-[:focus-within]/proc_app_release_step_pull_request_open:focus:fill-slate-200
    peer-[:focus-within]/proc_app_release_step_pull_request_open:active:fill-slate-300
    peer-[:focus-within]/proc_app_release_step_pull_request_open:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_release_step_pull_request_open:stroke-slate-200
    peer-[:focus-within]/proc_app_release_step_pull_request_open:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_release_step_pull_request_open:active:stroke-slate-400
    peer-[:focus-within]/proc_app_release_step_tag_and_push:[stroke-dasharray:3]
    peer-[:focus-within]/proc_app_release_step_tag_and_push:stroke-2
    peer-[:focus-within]/proc_app_release_step_tag_and_push:animate-[stroke-dashoffset-move_2s_linear_infinite]
    peer-[:focus-within]/proc_app_release_step_tag_and_push:hover:fill-slate-50
    peer-[:focus-within]/proc_app_release_step_tag_and_push:fill-slate-100
    peer-[:focus-within]/proc_app_release_step_tag_and_push:focus:fill-slate-200
    peer-[:focus-within]/proc_app_release_step_tag_and_push:active:fill-slate-300
    peer-[:focus-within]/proc_app_release_step_tag_and_push:hover:stroke-slate-100
    peer-[:focus-within]/proc_app_release_step_tag_and_push:stroke-slate-200
    peer-[:focus-within]/proc_app_release_step_tag_and_push:focus:stroke-slate-300
    peer-[:focus-within]/proc_app_release_step_tag_and_push:active:stroke-slate-400

  t_localhost_repo: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_localhost_repo_src: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_localhost_repo_target: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_localhost_repo_target_file_zip: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75
  t_localhost_repo_target_dist_dir: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-slate-200
    fill-slate-300
    focus:fill-slate-400
    active:fill-slate-500
    hover:stroke-slate-300
    stroke-slate-400
    focus:stroke-slate-500
    active:stroke-slate-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/tag_app_development:opacity-50
    peer-[:focus-within]/tag_deployment:opacity-75

  # Tags - act as group containers for highlighting associated things
  tag_app_development: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-emerald-400
    fill-emerald-500
    focus:fill-emerald-600
    active:fill-emerald-700
    hover:stroke-emerald-500
    stroke-emerald-600
    focus:stroke-emerald-700
    active:stroke-emerald-800
    [&>text]:fill-neutral-950
    peer/tag_app_development

  tag_deployment: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-emerald-400
    fill-emerald-500
    focus:fill-emerald-600
    active:fill-emerald-700
    hover:stroke-emerald-500
    stroke-emerald-600
    focus:stroke-emerald-700
    active:stroke-emerald-800
    [&>text]:fill-neutral-950
    peer/tag_deployment

  # Processes and process steps.
  #
  # Processes are group containers for their steps.
  #
  # Process steps are visible when parent process has focus, and are peers for things and edges
  # that interact within that step.
  #
  # Note: `process` nodes contain their steps' peer classes. This is because the process nodes will
  # be sibling elements to the `thing` / `edge_group` elements, whereas the process step nodes are
  # not sibling elements, so the `thing` and `edge_group` elements can only react to the `process`
  # nodes' state for the sibling selector to work.
  proc_app_dev: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-blue-400
    fill-blue-500
    focus:fill-blue-600
    active:fill-blue-700
    hover:stroke-blue-500
    stroke-blue-600
    focus:stroke-blue-700
    active:stroke-blue-800
    [&>text]:fill-neutral-950
    group/proc_app_dev
    peer/proc_app_dev_step_repository_clone
    peer/proc_app_dev_step_project_build
  proc_app_dev_step_repository_clone: |
    invisible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-400
    fill-sky-500
    focus:fill-sky-600
    active:fill-sky-700
    hover:stroke-sky-500
    stroke-sky-600
    focus:stroke-sky-700
    active:stroke-sky-800
    [&>text]:fill-neutral-950
    group-focus-within/proc_app_dev:visible
  proc_app_dev_step_project_build: |
    invisible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-400
    fill-sky-500
    focus:fill-sky-600
    active:fill-sky-700
    hover:stroke-sky-500
    stroke-sky-600
    focus:stroke-sky-700
    active:stroke-sky-800
    [&>text]:fill-neutral-950
    group-focus-within/proc_app_dev:visible

  proc_app_release: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-blue-400
    fill-blue-500
    focus:fill-blue-600
    active:fill-blue-700
    hover:stroke-blue-500
    stroke-blue-600
    focus:stroke-blue-700
    active:stroke-blue-800
    [&>text]:fill-neutral-950
    group/proc_app_release
    peer/proc_app_release_step_crate_version_update
    peer/proc_app_release_step_pull_request_open
    peer/proc_app_release_step_tag_and_push
    peer/proc_app_release_step_gh_actions_build
    peer/proc_app_release_step_gh_actions_publish
  proc_app_release_step_crate_version_update: |
    invisible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-400
    fill-sky-500
    focus:fill-sky-600
    active:fill-sky-700
    hover:stroke-sky-500
    stroke-sky-600
    focus:stroke-sky-700
    active:stroke-sky-800
    [&>text]:fill-neutral-950
    group-focus-within/proc_app_release:visible
  proc_app_release_step_pull_request_open: |
    invisible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-400
    fill-sky-500
    focus:fill-sky-600
    active:fill-sky-700
    hover:stroke-sky-500
    stroke-sky-600
    focus:stroke-sky-700
    active:stroke-sky-800
    [&>text]:fill-neutral-950
    group-focus-within/proc_app_release:visible
  proc_app_release_step_tag_and_push: |
    invisible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-400
    fill-sky-500
    focus:fill-sky-600
    active:fill-sky-700
    hover:stroke-sky-500
    stroke-sky-600
    focus:stroke-sky-700
    active:stroke-sky-800
    [&>text]:fill-neutral-950
    group-focus-within/proc_app_release:visible
  proc_app_release_step_gh_actions_build: |
    invisible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-400
    fill-sky-500
    focus:fill-sky-600
    active:fill-sky-700
    hover:stroke-sky-500
    stroke-sky-600
    focus:stroke-sky-700
    active:stroke-sky-800
    [&>text]:fill-neutral-950
    group-focus-within/proc_app_release:visible
  proc_app_release_step_gh_actions_publish: |
    invisible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-400
    fill-sky-500
    focus:fill-sky-600
    active:fill-sky-700
    hover:stroke-sky-500
    stroke-sky-600
    focus:stroke-sky-700
    active:stroke-sky-800
    [&>text]:fill-neutral-950
    group-focus-within/proc_app_release:visible

  proc_i12e_region_tier_app_deploy: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-blue-400
    fill-blue-500
    focus:fill-blue-600
    active:fill-blue-700
    hover:stroke-blue-500
    stroke-blue-600
    focus:stroke-blue-700
    active:stroke-blue-800
    [&>text]:fill-neutral-950
    group/proc_i12e_region_tier_app_deploy
    peer/proc_i12e_region_tier_app_deploy_step_ecs_service_update
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: |
    invisible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-sky-400
    fill-sky-500
    focus:fill-sky-600
    active:fill-sky-700
    hover:stroke-sky-500
    stroke-sky-600
    focus:stroke-sky-700
    active:stroke-sky-800
    [&>text]:fill-neutral-950
    group-focus-within/proc_i12e_region_tier_app_deploy:visible

  # Edges - visible when their associated step has focus
  #
  # If an edge is used as a dependency, then the `dependency` set of tailwind classes are applied,
  # and the edge is `visible` by default.
  # If it is also used as an interaction, then the `interaction` set of tailwind classes are
  # generated as peer classes to override the first set.
  #
  # If an edge is only used as an interaction, then the `interaction` set of tailwind classes are
  # generated as regular tailwind classes, and the edge is `invisible` by default.
  # The only peer class needed then is making the edge `visible` when the peer has focus.

  # dependency edges
  edge_dep_t_localhost__t_github_user_repo__pull: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-neutral-600
    fill-neutral-700
    focus:fill-neutral-800
    active:fill-neutral-900
    hover:stroke-neutral-700
    stroke-neutral-800
    focus:stroke-neutral-900
    active:stroke-neutral-950
    [&>text]:fill-neutral-950
  edge_dep_t_localhost__t_github_user_repo__pull__0: |
    stroke-1
  edge_dep_t_localhost__t_github_user_repo__pull__1: |
    stroke-1

  edge_dep_t_localhost__t_github_user_repo__push: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-neutral-600
    fill-neutral-700
    focus:fill-neutral-800
    active:fill-neutral-900
    hover:stroke-neutral-700
    stroke-neutral-800
    focus:stroke-neutral-900
    active:stroke-neutral-950
    [&>text]:fill-neutral-950
  edge_dep_t_localhost__t_github_user_repo__push__0: |
    stroke-1

  edge_dep_t_localhost__t_localhost__within: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-neutral-600
    fill-neutral-700
    focus:fill-neutral-800
    active:fill-neutral-900
    hover:stroke-neutral-700
    stroke-neutral-800
    focus:stroke-neutral-900
    active:stroke-neutral-950
    [&>text]:fill-neutral-950
  edge_dep_t_localhost__t_localhost__within__0: |
    stroke-1

  edge_dep_t_github_user_repo__t_github_user_repo__within: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-neutral-600
    fill-neutral-700
    focus:fill-neutral-800
    active:fill-neutral-900
    hover:stroke-neutral-700
    stroke-neutral-800
    focus:stroke-neutral-900
    active:stroke-neutral-950
    [&>text]:fill-neutral-950
  edge_dep_t_github_user_repo__t_github_user_repo__within__0: |
    stroke-1
  edge_dep_t_github_user_repo__t_github_user_repo__within__1: |
    stroke-1

  edge_dep_t_github_user_repo__t_aws_ecr_repo__push: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-neutral-600
    fill-neutral-700
    focus:fill-neutral-800
    active:fill-neutral-900
    hover:stroke-neutral-700
    stroke-neutral-800
    focus:stroke-neutral-900
    active:stroke-neutral-950
    [&>text]:fill-neutral-950
  edge_dep_t_github_user_repo__t_aws_ecr_repo__push__0: |
    stroke-1

  edge_dep_t_aws_ecr_repo__t_aws_ecs_service__push: |
    visible
    [stroke-dasharray:none]
    stroke-1
    hover:fill-neutral-600
    fill-neutral-700
    focus:fill-neutral-800
    active:fill-neutral-900
    hover:stroke-neutral-700
    stroke-neutral-800
    focus:stroke-neutral-900
    active:stroke-neutral-950
    [&>text]:fill-neutral-950
  edge_dep_t_aws_ecr_repo__t_aws_ecs_service__push__0: |
    stroke-1

  # interaction edges
  edge_ix_t_localhost__t_github_user_repo__pull: |
    invisible
    stroke-2
    hover:fill-blue-200
    fill-blue-300
    focus:fill-blue-400
    active:fill-blue-500
    hover:stroke-blue-300
    stroke-blue-400
    focus:stroke-blue-500
    active:stroke-blue-600
    [&>text]:fill-neutral-900
    peer-[:focus-within]/proc_app_dev_step_repository_clone:visible
    peer-[:focus-within]/proc_app_release_step_pull_request_open:visible
  edge_ix_t_localhost__t_github_user_repo__pull__0: |
    [stroke-dasharray:0,80,12,2,4,2,2,2,1,2,1,120]
    animate-[stroke-dashoffset-move-request_2s_linear_infinite]
  edge_ix_t_localhost__t_github_user_repo__pull__1: |
    [stroke-dasharray:0,120,1,2,1,2,2,2,4,2,8,2,20,80]
    animate-[stroke-dashoffset-move-response_2s_linear_infinite]

  edge_ix_t_localhost__t_github_user_repo__push: |
    invisible
    [stroke-dasharray:0,80,12,2,4,2,2,2,1,2,1,120]
    stroke-2
    hover:fill-violet-600
    fill-violet-700
    focus:fill-violet-800
    active:fill-violet-900
    hover:stroke-violet-700
    stroke-violet-800
    focus:stroke-violet-900
    active:stroke-violet-950
    [&>text]:fill-neutral-950
    peer-[:focus-within]/proc_app_release_step_tag_and_push:visible
  edge_ix_t_localhost__t_github_user_repo__push__0: |
    animate-[stroke-dashoffset-move-request_2s_linear_infinite]

  edge_ix_t_localhost__t_localhost__within: |
    invisible
    [stroke-dasharray:0,80,12,2,4,2,2,2,1,2,1,120]
    stroke-2
    hover:fill-violet-600
    fill-violet-700
    focus:fill-violet-800
    active:fill-violet-900
    hover:stroke-violet-700
    stroke-violet-800
    focus:stroke-violet-900
    active:stroke-violet-950
    [&>text]:fill-neutral-950
    peer-[:focus-within]/proc_app_dev_step_project_build:visible
    peer-[:focus-within]/proc_app_release_step_crate_version_update:visible
  edge_ix_t_localhost__t_localhost__within__0: |
    animate-[stroke-dashoffset-move-request_2s_linear_infinite]

  edge_ix_t_github_user_repo__t_github_user_repo__within: |
    invisible
    stroke-2
    hover:fill-violet-600
    fill-violet-700
    focus:fill-violet-800
    active:fill-violet-900
    hover:stroke-violet-700
    stroke-violet-800
    focus:stroke-violet-900
    active:stroke-violet-950
    [&>text]:fill-neutral-950
    peer-[:focus-within]/proc_app_release_step_gh_actions_build:visible
  edge_ix_t_github_user_repo__t_github_user_repo__within__0: |
    [stroke-dasharray:0,80,12,2,4,2,2,2,1,2,1,120]
    animate-[stroke-dashoffset-move-request_2s_linear_infinite]
  edge_ix_t_github_user_repo__t_github_user_repo__within__1: |
    [stroke-dasharray:0,120,1,2,1,2,2,2,4,2,8,2,20,80]
    animate-[stroke-dashoffset-move-response_2s_linear_infinite]

  edge_ix_t_github_user_repo__t_aws_ecr_repo__push: |
    invisible
    [stroke-dasharray:0,80,12,2,4,2,2,2,1,2,1,120]
    stroke-2
    hover:fill-violet-600
    fill-violet-700
    focus:fill-violet-800
    active:fill-violet-900
    hover:stroke-violet-700
    stroke-violet-800
    focus:stroke-violet-900
    active:stroke-violet-950
    [&>text]:fill-neutral-950
    peer-[:focus-within]/proc_app_release_step_gh_actions_publish:visible
  edge_ix_t_github_user_repo__t_aws_ecr_repo__push__0: |
    animate-[stroke-dashoffset-move-request_2s_linear_infinite]

  edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push: |
    invisible
    [stroke-dasharray:0,80,12,2,4,2,2,2,1,2,1,120]
    stroke-2
    hover:fill-violet-600
    fill-violet-700
    focus:fill-violet-800
    active:fill-violet-900
    hover:stroke-violet-700
    stroke-violet-800
    focus:stroke-violet-900
    active:stroke-violet-950
    [&>text]:fill-neutral-950
    peer-[:focus-within]/proc_i12e_region_tier_app_deploy_step_ecs_service_update:visible
  edge_ix_t_aws_ecr_repo__t_aws_ecs_service__push__0: |
    animate-[stroke-dashoffset-move-request_2s_linear_infinite]

# Layout configuration for nodes.
node_layout:
  # Root container
  #
  # This is in `column_reverse` order so tag node state controls `things` and `processes`:
  #
  # ```text
  # .------------------------------------.
  # | 1. _things_and_processes_container |
  # | -----------------------------------|
  # | 0. _tags_container                 |
  # '------------------------------------'
  # ```
  _root:
    flex:
      direction: "column_reverse"
      wrap: true
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0

  # Things and Processes container.
  #
  # This is in `row_reverse` order so process node state controls `things`:
  #
  # ```text
  # .------------------------------------------------.
  # | 1. _things_container | 0. _processes_container |
  # '------------------------------------------------'
  # ```
  _things_and_processes_container:
    flex:
      direction: "row_reverse"
      wrap: true
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0

  # Processes container (groups all processes horizontally)
  #
  # This is in `row` order so processes are laid out left to right:
  #
  # ```text
  # .-----------------------------------------------------------------------------.
  # | 0. proc_app_dev | 1. proc_app_release | 2. proc_i12e_region_tier_app_deploy |
  # '-----------------------------------------------------------------------------'
  # ```
  _processes_container:
    flex:
      direction: "row"
      wrap: true
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0

  # Individual processes (steps stacked vertically)
  #
  # This is in `column` order so steps are laid out top to bottom:
  #
  # ```text
  # .---------------------------------------.
  # | 0. proc_app_dev_step_repository_clone |
  # |---------------------------------------|
  # | 1. proc_app_dev_step_project_build    |
  # '---------------------------------------'
  # ```
  proc_app_dev:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  proc_app_dev_step_repository_clone: none
  proc_app_dev_step_project_build: none

  proc_app_release:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  proc_app_release_step_crate_version_update: none
  proc_app_release_step_pull_request_open: none
  proc_app_release_step_tag_and_push: none
  proc_app_release_step_gh_actions_build: none
  proc_app_release_step_gh_actions_publish: none

  proc_i12e_region_tier_app_deploy:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: none

  # Tags container
  #
  # This is in `row` order so tags are laid out left to right:
  #
  # ```text
  # .--------------------------------------------.
  # | 0. tag_app_development | 1. tag_deployment |
  # '--------------------------------------------'
  # ```
  _tags_container:
    flex:
      direction: "row"
      wrap: true
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0

  # Tags are leaves
  tag_app_development: none
  tag_deployment: none

  # Things container (top-level things arranged in a row)
  _things_container:
    flex:
      direction: "row"
      wrap: true
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0

  # Top-level things (children alternate between columns and rows)
  t_aws:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_aws_iam:
    flex:
      direction: "row"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_aws_iam_ecs_policy: none
  t_aws_ecr:
    flex:
      direction: "row"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_aws_ecr_repo:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_aws_ecr_repo_image_1: none
  t_aws_ecr_repo_image_2: none
  t_aws_ecs:
    flex:
      direction: "row"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_aws_ecs_cluster_app:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_aws_ecs_cluster_app_task: none

  t_github:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_github_user_repo: none

  t_localhost:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_localhost_repo:
    flex:
      direction: "row"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_localhost_repo_src: none
  t_localhost_repo_target:
    flex:
      direction: "column"
      wrap: false
      padding_top: 4.0
      padding_right: 4.0
      padding_bottom: 4.0
      padding_left: 4.0
      margin_top: 0.0
      margin_right: 0.0
      margin_bottom: 0.0
      margin_left: 0.0
      gap: 4.0
  t_localhost_repo_target_file_zip: none
  t_localhost_repo_target_dist_dir: none

# Additional CSS for animations.
css: |-
  @keyframes stroke-dashoffset-move {
    0%   { stroke-dasharray: 3; stroke-dashoffset: 30; }
    100% { stroke-dasharray: 3; stroke-dashoffset: 0; }
  }
  @keyframes stroke-dashoffset-move-request {
    0%   { stroke-dashoffset: 0; }
    100% { stroke-dashoffset: 228; }
  }
  @keyframes stroke-dashoffset-move-response {
    0%   { stroke-dashoffset: 0; }
    100% { stroke-dashoffset: -248; }
  }
````
