from tkinter import *
import math
import time
x=1
top = Tk()
C = Canvas(top, bg="black", height=400, width=400)
 
def dimEquiv(vec1,vec2):
    if len(vec1)==len(vec2):
        return True
    else:
        print("Error, vectors not dimensionally equivalent")

 
def ptp2vec(point1,point2): #point to point ==> vector
    vec=[]
    if dimEquiv(point1,point2):
        for dimension in range(len(point1)):
            vec.append(point2[dimension]-point1[dimension])
    return vec
 
 
def vectorSum(vec1, vec2):
    if dimEquiv(vec1,vec2):
        resultant=[]
        for dimension in range(len(vec1)):
            resultant.append(vec1[dimension]+vec2[dimension])
    return resultant
 
def scalar(vector, scalar):
    scaled=[]
    for dimension in range(len(vector)):
        scaled.append(vector[dimension]*scalar)
    return scaled

 
def magnitude(vector):
    squaredSum=0
    for dimension in range(len(vector)):
        squaredSum+=vector[dimension]**2
    return math.sqrt(squaredSum)
 
def dotProd(vec1, vec2):
    if dimEquiv(vec1,vec2):
        dotProd=0
        for dimension in range(len(vec1)):
            dotProd+=vec1[dimension]*vec2[dimension]
    return dotProd
 
def angle(vec1,vec2):
    dotProd1=dotProd(vec1,vec2)
    angle=math.acos(dotProd1/(magnitude(vec1)*magnitude(vec2)))
    return angle
 
def line(C,startPos,endPos):
    line = C.create_line(startPos[0],startPos[1],endPos,endPos, fill='white')
    C.pack()
 
def matrixMultiply(matrix1,matrix2):
    matrix=[]
    if len(matrix1[0])==len(matrix2):
        for rows in range(len(matrix1)):
            matrix.append([])
            for collumns in range(len(matrix2[0])):
                total=0
                for i in range(len(matrix2)):
                    total+=matrix1[rows][i]*matrix2[i][collumns]
                matrix[rows].append(total)
    else:
        print("matrices incompatible")
    return matrix
 
 
def xAxisRot(point,theta):
    return matrixMultiply([[1,0,0],[0,math.cos(theta),0-math.sin(theta)],[0,math.sin(theta),0-math.cos(theta)]],point)
 
def zAxisRot(point,theta):
    return matrixMultiply([[math.cos(theta),0-math.sin(theta),0],[math.sin(theta),math.cos(theta),0],[0,0,1]],point)
def triangle(C,point1,point2,point3):
    line(C,[point1[0],point1[1]],[point2[0],point2[1]])
    line(C,[point2[0],point2[1]],[point3[0],point3[1]])
    line(C,[point3[0],point3[1]],[point1[0],point1[1]])
 
theta=0
while True: #game loop
    C.delete("all")#clear frame
    #list of points for triangles in square
    points=[
        [[0],[0],[0]],
        [[100],[0],[0]],
        [[0],[100],[0]],
 
        [[100],[0],[0]],
        [[0],[100],[0]],
        [[100],[100],[0]]]
    theta+=0.0001
    for i in range(len(points)):
        points[i]=xAxisRot(points[i],theta)
        points[i]=zAxisRot(points[i],theta)
        #rotate around x and z axis with matrices
        points[i][1][0]+=200
        points[i][0][0]+=200
        #offsett to put origin in center screen
    triangle(C,points[0],points[1],points[2])
    triangle(C,points[3],points[4],points[5])
    top.update()#render new frame
top.mainloop()
 
