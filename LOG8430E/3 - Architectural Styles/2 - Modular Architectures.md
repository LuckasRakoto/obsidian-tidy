# Pipefilter
> Pipeline to prepare the data from the source to consumption data

For example,
`cat input.txt | grep "text" | sort > output.txt`
![[Pasted image 20250908095043.png]]
Another example can be compilers,
They take high level source code -> The compiler applies multiple filters and transformations (lexical analysis, syntactic analysis, code generation, etc.) to output the object file(s)
## Advantages
- Efficient: Filters can be deployed in parallel
- Independent from producers to consumers
- Reusable
- Possible to add or remove filters at runtime
## Disadvantages
- Possible bottleneck if a filter needs to wait for all its data
- Constrained to accept simple types of data
# Layered
> Each layers includes groups of methods, packages, classes, etc. They all capture a well-defined and disctinct responsability

For example,
UI - Services (application) - Logic (Domain) - Data Access (Persistance)

## Advantages
- Easy to implement & test
- Maintenance is easy
- Reusability
- Flexibility
- Security
## Disadvantages
- Not scalable, the layers are highly coupled
- Low performance: Data needs to travel through every layer to be processed
# MVC
Model layer: Responsible for the data
View layer: Responsible for the representation
Controller layer: Responsible for the logic
```
1. User interacts with the controller
2. Controller sends the changes to the model and the view (if necessary)
3. If the model has changed, the model sends the update to the View
```

Alternatives:
- MVP - Model-View-Presenter:
	- The user interacts with the view instead of the controller
- MVVM - Model-View-ViewModel
	- ViewModel control multiple views
![[Pasted image 20250908100517.png]]
- PAC - Presentaiton-Abstract-Controler:
	- Smaller MVCs in each module
## Advantages
- Separation of responsibilities (fast implementation, easy to test)
- The model can be shared among various views and controllers
## Disadvantages
- Architecture can increase complexity
# Blackboard
Scheduler that decides what process interacts with the problem
- The blackboard is a shared structured global memory
- Knowledge sources (KS) are specialized modules
- Scheduler configures and runs the modules (called KS)
## Advantages
- Highly specialized system
- Accomodate multiple expertises
## Disadvantages
- Scheduler becomes heavy and complex and a point of failure