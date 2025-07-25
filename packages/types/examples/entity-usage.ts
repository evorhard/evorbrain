/**
 * Example usage of entity interfaces with methods
 * This demonstrates how the entity interfaces can be used in the application
 */

import type {
  AreaWithMethods,
  GoalWithMethods,
  ProjectWithMethods,
  TaskWithMethods,
  TaskStatus,
  ProjectStatus
} from '../src/entities/index.js';

// Example: Working with Areas
async function areaExample(area: AreaWithMethods) {
  // Get all goals in this area
  const goals = await area.getGoals();
  console.log(`Area "${area.title}" has ${goals.length} goals`);
  
  // Calculate overall progress
  const progress = area.calculateProgress();
  console.log(`Overall progress: ${progress}%`);
  
  // Get active tasks across all goals and projects
  const activeTasks = await area.getTasksByStatus('in-progress');
  console.log(`Active tasks in area: ${activeTasks.length}`);
  
  // Check if area can be deleted
  if (area.canBeDeleted()) {
    console.log('Area can be safely deleted');
  } else {
    console.log('Area has active content and cannot be deleted');
  }
}

// Example: Working with Goals
async function goalExample(goal: GoalWithMethods) {
  // Check if goal is overdue
  if (goal.isOverdue()) {
    const daysOverdue = Math.abs(goal.getDaysUntilTarget() || 0);
    console.log(`Goal is ${daysOverdue} days overdue!`);
  }
  
  // Get all projects and calculate progress
  const projects = await goal.getProjects();
  const activeProjects = await goal.getActiveProjects();
  const progress = goal.calculateProgress();
  
  console.log(`Goal progress: ${progress}%`);
  console.log(`Active projects: ${activeProjects.length}/${projects.length}`);
  
  // Complete the goal if all projects are done
  if (goal.canBeCompleted()) {
    await goal.markAsCompleted();
    console.log('Goal completed!');
  }
}

// Example: Working with Projects
async function projectExample(project: ProjectWithMethods) {
  // Start a project
  if (project.canBeStarted()) {
    await project.start();
    console.log('Project started');
  }
  
  // Get task statistics
  const allTasks = await project.getTasks();
  const completedTasks = await project.getTasksByStatus('completed');
  const overdueTasks = await project.getOverdueTasks();
  
  console.log(`Tasks: ${completedTasks.length}/${allTasks.length} completed`);
  console.log(`Overdue tasks: ${overdueTasks.length}`);
  
  // Calculate estimated completion
  const estimatedDate = project.getEstimatedCompletionDate();
  if (estimatedDate) {
    console.log(`Estimated completion: ${estimatedDate.toLocaleDateString()}`);
  }
  
  // Update progress based on task completion
  const progress = project.calculateProgress();
  await project.updateProgress(progress);
}

// Example: Working with Tasks
async function taskExample(task: TaskWithMethods) {
  // Check task status
  if (task.isOverdue()) {
    console.log(`Task "${task.title}" is overdue!`);
  }
  
  // Start working on a task
  if (task.status === 'not-started') {
    await task.start();
    await task.startTimer();
    console.log('Started working on task');
  }
  
  // Work with subtasks
  if (task.hasSubtasks()) {
    const subtasks = await task.getSubtasks();
    console.log(`Task has ${subtasks.length} subtasks`);
  }
  
  // Complete a task
  if (task.canBeCompleted()) {
    await task.stopTimer();
    await task.complete();
    console.log(`Task completed in ${task.getTimeSpent()} minutes`);
    
    // Handle recurring tasks
    if (task.isRecurring()) {
      const nextInstance = await task.createNextInstance();
      if (nextInstance) {
        console.log('Created next recurring instance');
      }
    }
  }
  
  // Convert a complex task to a project
  if (task.hasSubtasks() && !task.projectId) {
    const project = await task.convertToProject();
    console.log(`Converted task to project: ${project.title}`);
  }
}

// Example: Cross-entity operations
async function crossEntityExample(
  area: AreaWithMethods,
  goal: GoalWithMethods,
  project: ProjectWithMethods,
  task: TaskWithMethods
) {
  // Add a new goal to an area
  const newGoal = await area.addGoal({
    title: 'New Health Goal',
    description: 'Improve overall fitness',
    targetDate: new Date(Date.now() + 90 * 24 * 60 * 60 * 1000).toISOString() // 90 days from now
  });
  
  // Add a project to the goal
  const newProject = await goal.addProject({
    title: 'Morning Exercise Routine',
    description: 'Establish consistent morning workouts',
    status: 'planning' as ProjectStatus
  });
  
  // Add tasks to the project
  const newTask = await project.addTask({
    title: 'Research workout plans',
    priority: 'high',
    dueDate: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString() // 1 week from now
  });
  
  // Create a recurring task
  const recurringTask = await project.addTask({
    title: 'Morning workout',
    priority: 'high',
    recurrence: {
      frequency: 'daily',
      interval: 1,
      weekDays: [1, 2, 3, 4, 5] // Monday to Friday
    }
  });
  
  console.log('Created full hierarchy from area to recurring task');
}

// Example: Validation and error handling
async function validationExample(task: TaskWithMethods) {
  // Validate before operations
  const errors = task.validate();
  if (errors.length > 0) {
    console.error('Task validation errors:', errors);
    return;
  }
  
  // Safe deletion with checks
  if (task.canBeDeleted()) {
    if (task.hasSubtasks()) {
      const subtasks = await task.getSubtasks();
      console.log(`Warning: Deleting task will also delete ${subtasks.length} subtasks`);
    }
    
    if (task.isRecurring()) {
      // Option to delete just this instance or the whole series
      await task.detachFromRecurrence();
      console.log('Detached from recurrence series before deletion');
    }
    
    await task.delete();
    console.log('Task deleted successfully');
  } else {
    console.log('Task cannot be deleted in current state');
  }
}

// Example: Working with entity history and metadata
async function metadataExample(project: ProjectWithMethods) {
  // Add tags for organization
  await project.addTag('high-priority');
  await project.addTag('q4-2025');
  
  // Check for specific tags
  if (project.hasTag('high-priority')) {
    console.log('This is a high priority project');
  }
  
  // Export for backup or sharing
  const markdown = project.toMarkdown();
  const json = project.toJSON();
  
  console.log('Exported project to markdown and JSON formats');
  
  // View history
  const history = await project.getHistory();
  console.log(`Project has ${history.length} versions`);
  
  // Revert to previous version if needed
  if (history.length > 1) {
    await project.revertTo(history[1].version);
    console.log('Reverted to previous version');
  }
}

export {
  areaExample,
  goalExample,
  projectExample,
  taskExample,
  crossEntityExample,
  validationExample,
  metadataExample
};