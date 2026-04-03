#-------------------------------------------------------------------------------
# Name:        Script para leitura de dados do gráfico desloc x tempo
# Author:      Vitor Freitas
# Date:        17/10/23
#-------------------------------------------------------------------------------

import numpy as np
import rustplotlib.pyplot as plt
import rustplotlib.font_manager as font_manager
from rustplotlib.ticker import FormatStrFormatter
import rustplotlib.patches as patches

#-------------------------------------------------------------------------------
# LEITURA DOS DADOS
#-------------------------------------------------------------------------------

skipline = 1 #Pular as primeiras linhas

def ler(file1):
    #Abre o arquivo
    f = open(file1, encoding='latin-1')
    #Pula o número de linhas colocado em skipline
    for i in range(skipline):
        next(f)
    # Lê cada linha do arquivo como um real(float) e armazenda em dados

    dados = [linha.strip() for linha in f]

    for i in range(len(dados)):
        if dados[i] == '%END':
            tamanho = i
            aux1 = []
            for j in range(tamanho):
                aux1.append(dados[j])

    #Separa a matriz de nós a cada vírgula, onde cada linha contém nó, cx1, cx2
    for i in range(tamanho):
        aux1[i] = aux1[i].split(',')

    # Converte a lista dados criada em um array(vetor) do tipo NumPy
    aux1 = np.array(aux1)

    #Elemento 2D: Colunas = (coordx e coordy)
    desloc = np.zeros(tamanho)
    tempo = np.zeros(tamanho)

    #Atribui os valores armazenados em aux1 e aux2 aos arrays de nodes e conec
    for i in range(tamanho):
        desloc[i] = aux1[i,0]
        tempo[i] = aux1[i,1]

    f.close()

    return desloc,tempo


#-------------------------------------------------------------------------------
# Temperatura ao longo do tempo
#-------------------------------------------------------------------------------

#Set de dados
temp_p1, time_p1 = ler('2perfil_temp_t=50s_DNS_cm.txt')
temp_p2, time_p2 = ler('2perfil_temp_t=500s_DNS_cm.txt')
#temp_p3, time_p3 = ler('2perfil_temp_t=1000s_DNS_cm.txt')

temp_p4, time_p4 = ler('2perfil_temp_t=50s_mscale_cm.txt')
temp_p5, time_p5 = ler('2perfil_temp_t=500s_mscale_cm.txt')
#temp_p6, time_p6 = ler('2perfil_temp_t=1000s_mscale_cm.txt')

plt.rcParams['font.family'] = ['serif']
plt.rcParams.update({'mathtext.fontset':'cm'})
fig, graf = plt.subplots(figsize=(6.5,4.5)) #Cria o GRAF como um subplot
mspac = 40 #Espaçamento dos marcadores

mask = time_p1 <= 0.08

#MSCALE
graf.plot(time_p4[mask],temp_p4[mask],color='black',marker='x',markevery=mspac,markersize=8,markerfacecolor='none',linewidth=1.0,label=r'Multiscale') #Faz o plot da curva, com os dados fornecidos
graf.plot(time_p5[mask],temp_p5[mask],color='black',marker='x',markevery=mspac,markersize=8,markerfacecolor='none',linewidth=1.0) #Faz o plot da curva, com os dados fornecidos
#graf.plot(time_p6[mask],temp_p6[mask],color='black',marker='x',markevery=mspac,markersize=8,markerfacecolor='none',linewidth=1.0) #Faz o plot da curva, com os dados fornecidos

#DNS
graf.plot(time_p1[mask],temp_p1[mask],color='BLUE',marker='s',markevery=mspac,markersize=0,markerfacecolor='none',linewidth=1.5,label=r'DNS') #Faz o plot da curva, com os dados fornecidos
graf.plot(time_p2[mask],temp_p2[mask],color='BLUE',marker='x',markevery=mspac,markersize=0,markerfacecolor='none',linewidth=1.5) #Faz o plot da curva, com os dados fornecidos
#graf.plot(time_p3[mask],temp_p3[mask],color='BLUE',marker='x',markevery=mspac,markersize=0,markerfacecolor='none',linewidth=1.5) #Faz o plot da curva, com os dados fornecidos


#graf.plot(time_p7,temp_p7,color='orange',marker='s',markevery=1,markersize=8,markerfacecolor='none',linewidth=0,label=r'(Reference)') #Faz o plot da curva, com os dados fornecidos
#graf.plot(time_p8,temp_p8,color='BLUE',marker='s',markevery=5,markersize=8,markerfacecolor='none',linewidth=0,label=r'(Reference)') #Faz o plot da curva, com os dados fornecidos
#graf.plot(time_p9,temp_p9,color='black',marker='s',markevery=5,markersize=8,markerfacecolor='none',linewidth=0,label=r'(Reference)') #Faz o plot da curva, com os dados fornecidos

font1 = {'family':'serif','color':'black','weight':'normal','size':12} #Configura uma fonte

#Configurações dos eixos
graf.set_xlabel(r'$x_1 (\text{m})$',fontdict=font1)  #Configura escrita do eixo x
graf.set_ylabel(r'$\theta\,(\text{K})$',fontdict=font1) #Configura escrita do eixo y
graf.set_xlim(left = -0.002,right = 0.082)
graf.set_ylim(bottom = -3.0, top = 103.0)
#graf.set_yticks(np.arange(0.0,100.0,5))
plt.yticks(fontsize = 10)
plt.xticks(fontsize = 10)

#Legenda
font1 = font_manager.FontProperties(family = 'serif', weight = 'normal', style = 'italic', size = 9)
graf.legend(loc='upper right', prop=font1)

#graf.invert_yaxis()
#graf.invert_xaxis()

graf.grid(color='grey',linestyle='dashed',linewidth=1.0,alpha=0.2) #Cria o gride do gráfico
font1 = font_manager.FontProperties(family = 'serif', weight = 'normal', style = 'italic', size = 12)
plt.text(0.003,6.0,r"$t = 50 s$",font=font1)
plt.text(0.012,25,r"$t = 500 s$",font=font1)
#plt.text(0.026,44,r"$t = 1000 s$",font=font1)

# rect = patches.Rectangle((0.017, 50), 0.003, 5, linewidth=1, edgecolor='red', facecolor='none')
# graf.add_patch(rect)

# graf2 = plt.axes([0.60, 0.4, .3, .3])
# graf2.spines['bottom'].set_color('red')
# graf2.spines['top'].set_color('red')
# graf2.spines['right'].set_color('red')
# graf2.spines['left'].set_color('red')
# graf2.spines['bottom'].set_lw(1)
# graf2.spines['top'].set_lw(1)
# graf2.spines['right'].set_lw(1)
# graf2.spines['left'].set_lw(1)

# graf2.plot(time_p6[mask],temp_p6[mask],color='black',marker='x',markevery=mspac,markersize=8,markerfacecolor='none',linewidth=1.0)
# graf2.plot(time_p3[mask],temp_p3[mask],color='BLUE',marker='x',markevery=mspac,markersize=0,markerfacecolor='none',linewidth=1.5)
# graf2.set_xlim(xmin=0.017, xmax=0.02)
# graf2.set_ylim(ymin=50.0, ymax=55.0)
# graf2.set_xticks([])
# graf2.set_yticks([])

plt.tight_layout() #Ajusta as dimensões do gráfico para caber tudo no plot
plt.show()
