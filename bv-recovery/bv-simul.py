import numpy as np
import math
import cv2
import matplotlib.pyplot as plt
N = 8 

# alpha(u) for dct matrix cofficients
def alpha(u) -> float:
    if u == 0:
        return math.sqrt(1/8)
    else:
        return math.sqrt(2/8)
    


def dct_coff(x: int,u: int) -> float:
    return alpha(u) * math.cos(((2*x+1)*u*math.pi)/(2*N))

def dct_matrix() -> np.ndarray:
    matrix = np.zeros((N,N))
    for u in range(N):
        for x in range(N):
            matrix[u,x] = round(dct_coff(u,x),5)
    return matrix

DCT_MATRIX = dct_matrix()


def alpha_vector(k,p,u,v,w):
    a  = np.array([DCT_MATRIX[w][k],DCT_MATRIX[w][p]])
    b = np.array([[DCT_MATRIX[u][k],DCT_MATRIX[u][p]],[DCT_MATRIX[v][k],DCT_MATRIX[v][p]]])
    try:
        inv_b = np.linalg.inv(b)
    except np.linalg.LinAlgError:
        print("zero alpha")
        return None
    return np.matmul(a,inv_b)



def a_threshold(alpha_one,alpha_two):
    return 0.01 * (1 + abs(alpha_one) + abs(alpha_two))

def dct2(block):
    """Applies 2D DCT to a block."""
    return cv2.dct(block.astype(np.float32)).round(3)

def idct2(block):
    """Applies 2D IDCT to a block."""
    return cv2.idct(block).round(3)

def corrupt_image(image):
    # Load a grayscale image
    corrupted_image = image.copy()
    count = 0
    for i in range(0,512,8):
        for j in range(0,512,8):
            block = image[i:i+8,j:j+8]
            dct_block = dct2(block)
            k  ,t , p ,q = 3 , 4 , 5, 6
            dct_block[k][t] = 1e6
            dct_block[p][q] = 2e7
            corrupted_block = idct2(dct_block)
            corrupted_image[i:i+8,j:j+8] = corrupted_block

    return corrupted_image 


def bv_algorithm_column(Auc,Avc,Awc , u,v,w , corrupt_block , column):
    # print(int(corrupt_block[w][column]))
    # print(int(Awc))
    # print(int(Awc) - int(corrupt_block[w][column]))c
    possible_locations = []
    Auc_e = int(Auc) - int(corrupt_block[u][column])
    Avc_e = int(Avc) - int(corrupt_block[v][column])
    Awc_e = int(Awc) - int(corrupt_block[w][column])
    print(Auc_e,Avc_e,Awc_e)
    error_vec = np.array([Auc_e,Avc_e])
    for k in range(0,8):
        for p in range(0,8):
            if k != p and p > k:
                alpha = alpha_vector(k,p,u,v,w) 
                A_thrs = a_threshold(alpha[0],alpha[1])
                val = (Awc_e - np.dot(alpha,error_vec))
                if val <= A_thrs:
                    possible_locations.append((k,p))

    print(possible_locations)
    return possible_locations


# gives the reftence pixels
def refrence_pixels():
    unique = []
    alphas = []
    for k in range(0,8):
        for p in range(0,8):
            if k != p:
               for u in range(0,8):
                   for v in range(0,8):
                       for w in range(0,8):
                            alpha = alpha_vector(k,p,u,v,w)
                            if alpha is not None:
                                if alphas  not in alphas:
                                    alphas.append(alpha)
                                    unique.append(unique)
    return unique



def main():
    
    image = cv2.imread("butterfly-icon.jpg", cv2.IMREAD_GRAYSCALE)
    corrupted_image = corrupt_image(image)

    corrupt_block = corrupted_image[0:8,0:8]
    refrence_block = image[0:8,0:8]
    u , v, w = 2,4,7 # the best case mentioned in the paper is  u = 2 v= 4  and w = 7
    column = 2
    Auc,Avc,Awc = refrence_block[u][column] , refrence_block[v][column] , refrence_block[w][column]
    bv_algorithm_column(Auc,Avc,Awc,u,v,w,corrupt_block,column)

    plt.figure(figsize=(12, 6))
    plt.subplot(1, 2, 1)
    plt.title("Original Image")
    plt.imshow(image, cmap='gray', vmin=0, vmax=255)

    plt.subplot(1, 2, 2)
    plt.title("Corrupted Image")
    plt.imshow(corrupted_image, cmap='gray', vmin=0, vmax=255)
